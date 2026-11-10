use crate::database::{Cache, DB};
use crate::entity::clients::Client;
use crate::http_client::{GuardedFetcher, consent_fetcher};
use crate::language::Language;
use crate::rauthy_config::{RauthyConfig, VarsConsent};
use rauthy_common::validation::{validate_consent_source_url, validate_https_url};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;
use std::future::Future;
use std::sync::{Arc, LazyLock, Mutex, PoisonError, Weak};
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::{error, warn};
use utoipa::ToSchema;

/// Max size of a remote consent document.
const REMOTE_CONSENT_MAX_SIZE: usize = 64 * 1024;
/// Max lifetime in seconds of a cached failed remote fetch, so an unavailable source is not
/// fetched again on every lookup. Capped by `consent.content_cache_ttl`.
const REMOTE_CONSENT_NEGATIVE_TTL: u32 = 60;
/// How long a lookup rejected by the fetcher's back pressure waits before it reads the cache
/// once more, in case another node or lookup has cached the document meanwhile.
const TOO_MANY_REQUESTS_RETRY_DELAY: Duration = Duration::from_millis(250);

/// Where the consent content for a client / resource comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentSource {
    /// Built-in: only the requested scope names, no fetch.
    Local,
    /// Fetched via `GET {url}?lang=<BCP47>`.
    Remote(Url),
}

impl ConsentSource {
    /// Parses `local` or a URL accepted by [`validate_consent_source_url`]: https, no userinfo,
    /// no fragment and no `lang` query key, which is appended on each fetch.
    pub fn parse(value: &str) -> Result<Self, String> {
        if value == "local" {
            return Ok(Self::Local);
        }
        validate_consent_source_url(value).map(Self::Remote)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ConsentSourceType {
    Local,
    Remote,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ConsentScope {
    pub name: String,
    /// Only for `remote`. `local` scopes are rendered from built-in translations.
    pub title: Option<String>,
    pub description: Option<String>,
    pub warning: bool,
}

/// What the user is asked to consent to for a login.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ConsentContent {
    /// `local` as well for an unavailable `remote` source.
    pub source: ConsentSourceType,
    /// SHA-256 (hex) of the canonical JSON of all other fields, i.e. of the content as shown,
    /// the version a consent is given for. It depends on the user's language and the requested
    /// scopes.
    pub content_hash: String,
    /// Borrowed from the `[[consent.sources]]` registry, owned from a remote document.
    #[schema(value_type = Option<String>)]
    pub name: Option<Cow<'static, str>>,
    /// The requested scopes, sorted.
    pub scopes: Vec<ConsentScope>,
    pub intro: Option<String>,
    pub terms_url: Option<String>,
}

impl ConsentContent {
    /// The consent content for a login to `client` with the optional RFC 8707 `resource`.
    /// `None` without `consent.enable`. An unavailable or invalid remote source degrades to
    /// the `local` content and never fails the login. Not called yet, the consent screen will.
    ///
    /// For a remote source, the content is shown from the `lang` document, or the `en` one if
    /// that fails, with the scope warnings of the `en` document added. The `content_hash` is
    /// the one of the content as shown, so any change of a shown field asks for consent again,
    /// and so may a change of the user's language. Without a valid `en` document, the whole
    /// content is `local`.
    pub async fn resolve(
        client: &Client,
        resource: Option<&str>,
        scopes: &[String],
        lang: Language,
    ) -> Option<Self> {
        Self::resolve_with(
            client,
            resource,
            scopes,
            lang,
            &RauthyConfig::get().vars.consent,
            RemoteConsent::find,
        )
        .await
    }

    /// [`Self::resolve`] with injected config and remote lookup.
    async fn resolve_with<F>(
        client: &Client,
        resource: Option<&str>,
        scopes: &[String],
        lang: Language,
        vars: &'static VarsConsent,
        find_remote: F,
    ) -> Option<Self>
    where
        F: AsyncFn(&Url, Language) -> Result<RemoteConsent, ErrorResponse>,
    {
        let source = client.consent_source_with(resource, vars)?;
        // the registry entry is the source whenever it exists for the `resource`
        let name = resource
            .and_then(|r| vars.source_for(r))
            .map(|s| Cow::Borrowed(s.name.as_str()));

        let ConsentSource::Remote(url) = source else {
            return Some(Self::build(None, name, scopes));
        };

        // `None` for `lang == en`. Fetched concurrently with `en`, and ignored if that one
        // fails.
        let (en, translation) = if lang == DEFAULT_LANGUAGE {
            (find_remote(&url, DEFAULT_LANGUAGE).await, None)
        } else {
            let (en, translation) =
                tokio::join!(find_remote(&url, DEFAULT_LANGUAGE), find_remote(&url, lang));
            (en, Some(translation))
        };

        let Ok(en) = en.inspect_err(|err| {
            warn!(
                "Consent source {url} unavailable for client {}, using `local`: {}",
                client.id, err.message
            )
        }) else {
            return Some(Self::build(None, name, scopes));
        };
        let remote = match translation {
            None => RemoteDocs::new(en, None),
            Some(Ok(display)) => RemoteDocs::new(display, Some(en)),
            Some(Err(err)) => {
                warn!(
                    "Consent source {url} unavailable for client {} and language {}, using \
                    `{}`: {}",
                    client.id,
                    lang.as_bcp47(),
                    DEFAULT_LANGUAGE.as_bcp47(),
                    err.message
                );
                RemoteDocs::new(en, None)
            }
        };

        Some(Self::build(Some(remote), name, scopes))
    }

    /// `name` from the `[[consent.sources]]` registry wins over the remote document's `name`.
    fn build(
        remote: Option<RemoteDocs>,
        name: Option<Cow<'static, str>>,
        scopes: &[String],
    ) -> Self {
        let mut scopes = scopes.iter().map(String::as_str).collect::<Vec<_>>();
        scopes.sort_unstable();
        scopes.dedup();

        let Some(RemoteDocs {
            display: mut remote,
            en_warnings,
        }) = remote
        else {
            return Self::local(name, &scopes);
        };

        let scopes = scopes
            .into_iter()
            .map(|s| {
                // a stale translation must not drop a warning of the `en` document
                let en_warning = en_warnings.contains(s);
                // the scopes are deduplicated, so each one is taken once
                match remote.scopes.remove(s) {
                    Some(scope) => ConsentScope {
                        name: s.to_string(),
                        title: Some(scope.title),
                        description: scope.description,
                        warning: scope.warning || en_warning,
                    },
                    None => ConsentScope {
                        name: s.to_string(),
                        title: None,
                        description: None,
                        warning: en_warning,
                    },
                }
            })
            .collect();

        Self::hashed(
            ConsentSourceType::Remote,
            name.or(remote.name.map(Cow::Owned)),
            scopes,
            remote.intro,
            remote.terms_url,
        )
    }

    fn local(name: Option<Cow<'static, str>>, scopes: &[&str]) -> Self {
        let scopes = scopes
            .iter()
            .map(|s| ConsentScope {
                name: s.to_string(),
                title: None,
                description: None,
                warning: false,
            })
            .collect();
        Self::hashed(ConsentSourceType::Local, name, scopes, None, None)
    }

    /// The content with the `content_hash` of all its other fields.
    fn hashed(
        source: ConsentSourceType,
        name: Option<Cow<'static, str>>,
        scopes: Vec<ConsentScope>,
        intro: Option<String>,
        terms_url: Option<String>,
    ) -> Self {
        let mut slf = Self {
            source,
            content_hash: String::default(),
            name,
            scopes,
            intro,
            terms_url,
        };
        let mut value = serde_json::to_value(&slf).expect("ConsentContent to always serialize");
        if let Value::Object(map) = &mut value {
            map.remove("content_hash");
        }
        slf.content_hash = content_hash(&value);
        slf
    }
}

/// The language a remote source must provide. Shown when the user's language is unavailable.
const DEFAULT_LANGUAGE: Language = Language::En;

/// The remote document shown for a login, in the user's language (or `en` as fallback), and
/// the scopes with a warning in the `en` document, which are shown with a warning as well.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RemoteDocs {
    display: RemoteConsent,
    /// Empty if `display` is the `en` document.
    en_warnings: BTreeSet<String>,
}

impl RemoteDocs {
    /// `en` is `None` if `display` is the `en` document.
    fn new(display: RemoteConsent, en: Option<RemoteConsent>) -> Self {
        let en_warnings = en
            .map(|en| {
                en.scopes
                    .into_iter()
                    .filter_map(|(name, scope)| scope.warning.then_some(name))
                    .collect()
            })
            .unwrap_or_default();
        Self {
            display,
            en_warnings,
        }
    }

    #[cfg(test)]
    fn same(doc: RemoteConsent) -> Self {
        Self::new(doc, None)
    }
}

/// Response of `GET {url}?lang=<BCP47>` for a remote consent source. Unknown fields are ignored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RemoteConsent {
    /// Only used without a `[[consent.sources]]` `name`, i.e. for the client's own source.
    name: Option<String>,
    #[serde(default)]
    scopes: BTreeMap<String, RemoteConsentScope>,
    intro: Option<String>,
    terms_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RemoteConsentScope {
    title: String,
    description: Option<String>,
    #[serde(default)]
    warning: bool,
}

impl RemoteConsent {
    #[inline]
    fn cache_idx(url: &Url, lang: Language) -> String {
        format!("{}_{}", lang.as_bcp47(), url.as_str())
    }

    /// Cached [`Self::fetch`] via the consent source fetcher. A failed fetch is cached as
    /// well, for at most [`REMOTE_CONSENT_NEGATIVE_TTL`]. Concurrent lookups of the same
    /// uncached document share one fetch.
    async fn find(url: &Url, lang: Language) -> Result<Self, ErrorResponse> {
        Self::find_with(
            &HqlConsentCache,
            &IN_FLIGHT,
            url,
            lang,
            RauthyConfig::get().vars.consent.content_cache_ttl,
            || Self::fetch(consent_fetcher(), url, lang),
        )
        .await
    }

    /// [`Self::find`] with an injected cache, in-flight map, TTL and fetch.
    async fn find_with<C, F, Fut>(
        cache: &C,
        in_flight: &InFlight,
        url: &Url,
        lang: Language,
        ttl: u32,
        fetch: F,
    ) -> Result<Self, ErrorResponse>
    where
        C: ConsentCache,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Self, ErrorResponse>>,
    {
        let idx = Self::cache_idx(url, lang);
        if let Some(res) = Self::cached(cache, &idx, url).await {
            return res;
        }

        let res = {
            let shared = in_flight.join(&idx);
            shared
                .cell()
                .get_or_init(|| async {
                    // a shared lookup that ended just before may have cached it already
                    if let Some(res) = Self::cached(cache, &idx, url).await {
                        return res;
                    }
                    let res = fetch().await;
                    Self::store(cache, &idx, url, &res, ttl).await;
                    res
                })
                .await
                .clone()
        };

        if matches!(&res, Err(err) if matches!(err.error, ErrorResponseType::TooManyRequests(_))) {
            tokio::time::sleep(TOO_MANY_REQUESTS_RETRY_DELAY).await;
            if let Some(Ok(slf)) = Self::cached(cache, &idx, url).await {
                return Ok(slf);
            }
        }
        res
    }

    /// The cached lookup result, `None` on a cache miss or an unreadable cache.
    async fn cached<C: ConsentCache>(
        cache: &C,
        idx: &str,
        url: &Url,
    ) -> Option<Result<Self, ErrorResponse>> {
        match cache.get(idx).await {
            Ok(Some(Some(slf))) => Some(Ok(slf)),
            Ok(Some(None)) => Some(Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                "consent source recently unavailable",
            ))),
            Ok(None) => None,
            Err(err) => {
                error!("Cannot read consent content cache for {url}: {err}");
                None
            }
        }
    }

    /// Caches the result of a fetch, see [`cache_value`] and [`keeps_cached`].
    async fn store<C: ConsentCache>(
        cache: &C,
        idx: &str,
        url: &Url,
        res: &Result<Self, ErrorResponse>,
        ttl: u32,
    ) {
        let Some((value, ttl)) = cache_value(res, ttl) else {
            return;
        };
        // The remaining race between this read and the write is acceptable.
        if value.is_none() && keeps_cached(&cache.get(idx).await) {
            return;
        }
        if let Err(err) = cache.put(idx, value, ttl).await {
            error!("Cannot cache consent content for {url}: {err}");
        }
    }

    async fn fetch(
        fetcher: &GuardedFetcher,
        url: &Url,
        lang: Language,
    ) -> Result<Self, ErrorResponse> {
        let mut url = url.clone();
        url.query_pairs_mut().append_pair("lang", lang.as_bcp47());

        let bytes = fetcher.fetch_bounded(url, REMOTE_CONSENT_MAX_SIZE).await?;
        Self::parse(&bytes)
    }

    fn parse(bytes: &[u8]) -> Result<Self, ErrorResponse> {
        let slf = serde_json::from_slice::<Self>(bytes).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("invalid consent document: {err}"),
            )
        })?;

        if let Some(url) = &slf.terms_url {
            validate_https_url(url).map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("invalid consent document `terms_url`: {err}"),
                )
            })?;
        }

        Ok(slf)
    }
}

/// The cache value and TTL for the result of a fetch. `None` for the local back pressure of
/// the fetcher, which says nothing about the source.
fn cache_value(
    res: &Result<RemoteConsent, ErrorResponse>,
    ttl: u32,
) -> Option<(Option<&RemoteConsent>, u32)> {
    match res {
        Ok(slf) => Some((Some(slf), ttl)),
        Err(err) if matches!(err.error, ErrorResponseType::TooManyRequests(_)) => None,
        Err(_) => Some((None, negative_cache_ttl(ttl))),
    }
}

/// `true` if a failed fetch must not overwrite what the cache holds, given a cache read right
/// before the write: a value a concurrent lookup has cached meanwhile is kept. An unreadable
/// cache is written like an empty one, so the failure is still cached.
fn keeps_cached<E>(reread: &Result<Option<Option<RemoteConsent>>, E>) -> bool {
    matches!(reread, Ok(Some(_)))
}

/// Cache of [`RemoteConsent::find`]. A value is the document, or `None` for a recently
/// unavailable source.
trait ConsentCache {
    async fn get(&self, idx: &str) -> Result<Option<Option<RemoteConsent>>, ErrorResponse>;

    async fn put(
        &self,
        idx: &str,
        value: Option<&RemoteConsent>,
        ttl: u32,
    ) -> Result<(), ErrorResponse>;
}

/// [`ConsentCache`] in [`Cache::ConsentContent`].
struct HqlConsentCache;

impl ConsentCache for HqlConsentCache {
    async fn get(&self, idx: &str) -> Result<Option<Option<RemoteConsent>>, ErrorResponse> {
        Ok(DB::hql().get(Cache::ConsentContent, idx).await?)
    }

    async fn put(
        &self,
        idx: &str,
        value: Option<&RemoteConsent>,
        ttl: u32,
    ) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(
                Cache::ConsentContent,
                idx.to_string(),
                &value,
                Some(ttl as i64),
            )
            .await?;
        Ok(())
    }
}

/// A lookup result shared by concurrent lookups of the same document.
type SharedResult = OnceCell<Result<RemoteConsent, ErrorResponse>>;
type SharedLookup = Arc<SharedResult>;

/// The remote lookups in flight, by cache key.
static IN_FLIGHT: LazyLock<InFlight> = LazyLock::new(InFlight::default);

/// Shared remote lookups by cache key, so concurrent lookups of the same uncached document
/// share one fetch. The map holds weak references only, and the last [`InFlightLookup`] of a
/// key removes its entry, also when it is cancelled. So the map never holds more keys than
/// lookups are waiting.
#[derive(Default)]
struct InFlight(Mutex<HashMap<String, Weak<SharedResult>>>);

impl InFlight {
    /// Joins the shared lookup for `idx`, or starts a new one.
    fn join(&self, idx: &str) -> InFlightLookup<'_> {
        let mut map = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let cell = match map.get(idx).and_then(Weak::upgrade) {
            Some(cell) => cell,
            None => {
                let cell = SharedLookup::default();
                map.insert(idx.to_string(), Arc::downgrade(&cell));
                cell
            }
        };
        InFlightLookup {
            in_flight: self,
            idx: idx.to_string(),
            cell: Some(cell),
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).len()
    }
}

/// A lookup waiting on a shared one. The last one of a key removes its map entry on drop.
struct InFlightLookup<'a> {
    in_flight: &'a InFlight,
    idx: String,
    cell: Option<SharedLookup>,
}

impl InFlightLookup<'_> {
    fn cell(&self) -> &SharedResult {
        self.cell.as_ref().expect("cell to exist until drop")
    }
}

impl Drop for InFlightLookup<'_> {
    fn drop(&mut self) {
        drop(self.cell.take());
        let mut map = self
            .in_flight
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        // a newer lookup may have replaced the entry meanwhile
        if map
            .get(&self.idx)
            .is_some_and(|cell| cell.strong_count() == 0)
        {
            map.remove(&self.idx);
        }
    }
}

/// TTL of a cached failed remote fetch: `min(content_cache_ttl, 60s)`.
#[inline]
fn negative_cache_ttl(content_cache_ttl: u32) -> u32 {
    content_cache_ttl.min(REMOTE_CONSENT_NEGATIVE_TTL)
}

/// SHA-256 (hex) of the [`canonical_json`] of `value`.
fn content_hash(value: &Value) -> String {
    hex::encode(hmac_sha256::Hash::hash(canonical_json(value).as_bytes()))
}

/// Compact JSON with object keys sorted by their UTF-8 bytes, recursively.
fn canonical_json(value: &Value) -> String {
    let mut out = String::with_capacity(128);
    write_canonical(value, &mut out);
    out
}

fn write_canonical(value: &Value, out: &mut String) {
    match value {
        Value::Object(map) => {
            let mut entries = map.iter().collect::<Vec<_>>();
            entries.sort_unstable_by_key(|(key, _)| *key);

            out.push('{');
            for (i, (key, value)) in entries.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                // `Value` display escapes the key
                write!(out, "{}:", Value::from(key.as_str())).expect("String write to succeed");
                write_canonical(value, out);
            }
            out.push('}');
        }
        Value::Array(arr) => {
            out.push('[');
            for (i, value) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_canonical(value, out);
            }
            out.push(']');
        }
        value => write!(out, "{value}").expect("String write to succeed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rauthy_config::VarsConsentSource;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn strs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    const REMOTE: &str = r#"{
        "name": "Example API",
        "scopes": {
            "data.read": {"title": "Read data", "description": "Read your data"},
            "data.write": {"title": "Write data", "description": "Change your data", "warning": true}
        },
        "intro": "Example wants to access your data.",
        "terms_url": "https://example.com/terms"
    }"#;

    // same content, different key order and whitespace
    const REMOTE_REORDERED: &str = r#"{"terms_url":"https://example.com/terms",
        "intro":"Example wants to access your data.",
        "scopes":{"data.write":{"warning":true,"description":"Change your data","title":"Write data"},
        "data.read":{"description":"Read your data","title":"Read data","warning":false}},
        "name":"Example API","unknown":42}"#;

    #[test]
    fn parse_consent_source() {
        assert_eq!(ConsentSource::parse("local").unwrap(), ConsentSource::Local);
        assert_eq!(
            ConsentSource::parse("https://consent.example.com/api").unwrap(),
            ConsentSource::Remote(Url::parse("https://consent.example.com/api").unwrap())
        );
        assert!(ConsentSource::parse("https://consent.example.com/api?v=1&language=x").is_ok());

        for v in [
            "",
            "Local",
            "http://consent.example.com",
            "https://",
            "https://consent.example.com/#x",
            "consent.example.com",
            "file:///etc/passwd",
            "https://consent.example.com/api?lang=en",
            "https://consent.example.com/api?v=1&lang=",
        ] {
            assert!(ConsentSource::parse(v).is_err(), "{v}");
        }
    }

    #[test]
    fn parse_consent_source_rejects_userinfo() {
        for v in [
            "https://u:p@consent.example.com/api",
            "https://u@consent.example.com/api",
            "https://:p@consent.example.com/api",
        ] {
            assert!(ConsentSource::parse(v).is_err(), "{v}");
        }
    }

    #[test]
    fn canonical_json_sorts_keys() {
        let a = serde_json::from_str::<Value>(r#"{"b": [3, {"z": null, "a": "x"}], "a": 1.5}"#)
            .unwrap();
        let b = serde_json::from_str::<Value>(r#"{"a":1.5,"b":[3,{"a":"x","z":null}]}"#).unwrap();
        assert_eq!(
            canonical_json(&a),
            r#"{"a":1.5,"b":[3,{"a":"x","z":null}]}"#
        );
        assert_eq!(canonical_json(&a), canonical_json(&b));

        // keys are escaped like values
        let v = serde_json::json!({"q\"k": "v\n"});
        assert_eq!(canonical_json(&v), r#"{"q\"k":"v\n"}"#);
    }

    #[test]
    fn local_content_hash() {
        let content = ConsentContent::build(None, None, &strs(&["openid", "email", "openid"]));
        assert_eq!(content.source, ConsentSourceType::Local);
        assert_eq!(
            content.content_hash,
            hex::encode(hmac_sha256::Hash::hash(
                concat!(
                    r#"{"intro":null,"name":null,"scopes":["#,
                    r#"{"description":null,"name":"email","title":null,"warning":false},"#,
                    r#"{"description":null,"name":"openid","title":null,"warning":false}],"#,
                    r#""source":"local","terms_url":null}"#
                )
                .as_bytes()
            ))
        );
        assert_eq!(
            content
                .scopes
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            vec!["email", "openid"]
        );
        assert!(content.scopes.iter().all(|s| s.title.is_none()));

        // independent of the request order
        let other = ConsentContent::build(None, None, &strs(&["email", "openid"]));
        assert_eq!(content.content_hash, other.content_hash);
        // but not of the scopes
        let other = ConsentContent::build(None, None, &strs(&["email", "openid", "profile"]));
        assert_ne!(content.content_hash, other.content_hash);
    }

    #[test]
    fn remote_content() {
        let remote = RemoteConsent::parse(REMOTE.as_bytes()).unwrap();
        let content = ConsentContent::build(
            Some(RemoteDocs::same(remote)),
            Some("Registry".into()),
            &strs(&["openid", "data.write", "data.read"]),
        );

        assert_eq!(content.source, ConsentSourceType::Remote);
        // the registry name wins over the remote one
        assert_eq!(content.name.as_deref(), Some("Registry"));
        assert_eq!(
            content.intro.as_deref(),
            Some("Example wants to access your data.")
        );
        assert_eq!(
            content.terms_url.as_deref(),
            Some("https://example.com/terms")
        );
        assert_eq!(
            content.scopes,
            vec![
                ConsentScope {
                    name: "data.read".to_string(),
                    title: Some("Read data".to_string()),
                    description: Some("Read your data".to_string()),
                    warning: false,
                },
                ConsentScope {
                    name: "data.write".to_string(),
                    title: Some("Write data".to_string()),
                    description: Some("Change your data".to_string()),
                    warning: true,
                },
                ConsentScope {
                    name: "openid".to_string(),
                    title: None,
                    description: None,
                    warning: false,
                },
            ]
        );

        let local =
            ConsentContent::build(None, None, &strs(&["openid", "data.write", "data.read"]));
        assert_ne!(content.content_hash, local.content_hash);

        // without a registry entry, the remote name is used
        let remote = RemoteConsent::parse(REMOTE.as_bytes()).unwrap();
        let content =
            ConsentContent::build(Some(RemoteDocs::same(remote)), None, &strs(&["data.read"]));
        assert_eq!(content.name.as_deref(), Some("Example API"));
    }

    #[test]
    fn negative_cache_ttl_is_capped() {
        assert_eq!(negative_cache_ttl(3600), 60);
        assert_eq!(negative_cache_ttl(60), 60);
        assert_eq!(negative_cache_ttl(10), 10);
        assert_eq!(negative_cache_ttl(1), 1);
    }

    #[test]
    fn remote_content_hash_is_deterministic() {
        let scopes = strs(&["data.read"]);
        let a = ConsentContent::build(
            Some(RemoteDocs::same(
                RemoteConsent::parse(REMOTE.as_bytes()).unwrap(),
            )),
            None,
            &scopes,
        );
        let b = ConsentContent::build(
            Some(RemoteDocs::same(
                RemoteConsent::parse(REMOTE_REORDERED.as_bytes()).unwrap(),
            )),
            None,
            &scopes,
        );
        assert_eq!(a.content_hash, b.content_hash);

        let changed = REMOTE.replace("Read your data", "Read all your data");
        let c = ConsentContent::build(
            Some(RemoteDocs::same(
                RemoteConsent::parse(changed.as_bytes()).unwrap(),
            )),
            None,
            &scopes,
        );
        assert_ne!(a.content_hash, c.content_hash);
    }

    #[test]
    fn remote_consent_parse_rejects_invalid() {
        for doc in [
            "",
            "[]",
            r#"{"scopes": {"a": {"description": "no title"}}}"#,
            r#"{"terms_url": "javascript:alert(1)"}"#,
            r#"{"terms_url": "http://example.com/terms"}"#,
        ] {
            assert!(RemoteConsent::parse(doc.as_bytes()).is_err(), "{doc}");
        }
        assert!(RemoteConsent::parse(b"{}").is_ok());
    }

    fn test_fetcher() -> GuardedFetcher {
        GuardedFetcher::for_tests(
            true,
            reqwest::Client::builder().timeout(Duration::from_secs(5)),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn remote_fetch_sends_lang() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 4096];
            let n = socket.read(&mut buf).await.unwrap();
            let head = String::from_utf8_lossy(&buf[..n]).to_string();
            let res = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{REMOTE}",
                REMOTE.len()
            );
            socket.write_all(res.as_bytes()).await.unwrap();
            socket.shutdown().await.unwrap();
            head
        });

        let url = Url::parse(&format!("http://127.0.0.1:{port}/consent?v=1")).unwrap();
        let remote = RemoteConsent::fetch(&test_fetcher(), &url, Language::ZhHans)
            .await
            .unwrap();
        assert_eq!(remote, RemoteConsent::parse(REMOTE.as_bytes()).unwrap());

        let head = server.await.unwrap();
        assert!(
            head.starts_with("GET /consent?v=1&lang=zh-Hans HTTP/1.1"),
            "{head}"
        );
    }

    /// `resolve_with` borrows the config for `'static`, like the global one.
    fn vars(sources: Vec<VarsConsentSource>) -> &'static VarsConsent {
        leak(VarsConsent {
            enable: true,
            content_cache_ttl: 3600,
            danger_allow_private_addresses: false,
            sources: sources
                .into_iter()
                .map(|s| (s.resource.clone(), s))
                .collect(),
        })
    }

    /// A test config that lives for `'static`. Tests build only a few.
    fn leak(vars: VarsConsent) -> &'static VarsConsent {
        Box::leak(Box::new(vars))
    }

    #[tokio::test]
    async fn resolve_degrades_to_local_on_remote_failure() {
        // a port nobody listens on
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        let url = format!("https://127.0.0.1:{port}/consent");

        let resource = "https://api.example.com";
        let vars = vars(vec![
            VarsConsentSource::new(resource.to_string(), &url, "Registry".to_string()).unwrap(),
        ]);
        let client = Client {
            id: "client".to_string(),
            consent: Some(url.clone()),
            ..Default::default()
        };
        let scopes = strs(&["openid", "data.read"]);
        let fetcher = test_fetcher();
        let find = async |url: &Url, lang| RemoteConsent::fetch(&fetcher, url, lang).await;

        // registry source
        let content = ConsentContent::resolve_with(
            &client,
            Some(resource),
            &scopes,
            Language::En,
            vars,
            find,
        )
        .await
        .unwrap();
        assert_eq!(
            content,
            ConsentContent::build(None, Some("Registry".into()), &scopes)
        );
        assert_eq!(content.source, ConsentSourceType::Local);
        assert_eq!(content.name.as_deref(), Some("Registry"));

        // the client's own source
        let content =
            ConsentContent::resolve_with(&client, None, &scopes, Language::En, vars, find)
                .await
                .unwrap();
        assert_eq!(content, ConsentContent::build(None, None, &scopes));
    }

    #[tokio::test]
    async fn resolve_remote() {
        let resource = "https://api.example.com";
        let url = "https://consent.example.com/api";
        let vars = vars(vec![
            VarsConsentSource::new(resource.to_string(), url, "Registry".to_string()).unwrap(),
        ]);
        let client = Client {
            id: "client".to_string(),
            consent: Some("https://consent.example.com/client".to_string()),
            ..Default::default()
        };
        let scopes = strs(&["data.read"]);
        let find = async |url: &Url, _lang: Language| {
            Ok(RemoteConsent {
                name: Some(url.to_string()),
                ..RemoteConsent::parse(REMOTE.as_bytes()).unwrap()
            })
        };

        // registry: its name wins
        let content = ConsentContent::resolve_with(
            &client,
            Some(resource),
            &scopes,
            Language::De,
            vars,
            find,
        )
        .await
        .unwrap();
        assert_eq!(content.source, ConsentSourceType::Remote);
        assert_eq!(content.name.as_deref(), Some("Registry"));

        // client column: the remote name is used
        let content =
            ConsentContent::resolve_with(&client, None, &scopes, Language::De, vars, find)
                .await
                .unwrap();
        assert_eq!(content.source, ConsentSourceType::Remote);
        assert_eq!(
            content.name.as_deref(),
            Some("https://consent.example.com/client")
        );

        // disabled
        let disabled = leak(VarsConsent {
            enable: false,
            sources: vars.sources.clone(),
            ..*vars
        });
        assert!(
            ConsentContent::resolve_with(&client, None, &scopes, Language::De, disabled, find)
                .await
                .is_none()
        );
    }

    const REMOTE_DE: &str = r#"{
        "name": "Beispiel-API",
        "scopes": {
            "data.read": {"title": "Daten lesen", "description": "Deine Daten lesen"}
        },
        "intro": "Beispiel möchte auf deine Daten zugreifen.",
        "terms_url": "https://example.com/terms?lang=de"
    }"#;

    /// Resolves the client's own remote source with `en` / `de` mock documents, `None` for
    /// an unavailable one.
    async fn resolve_mock(lang: Language, en: Option<&str>, de: Option<&str>) -> ConsentContent {
        resolve_mock_counted(lang, en, de).await.0
    }

    /// [`resolve_mock`], with the number of remote lookups made.
    async fn resolve_mock_counted(
        lang: Language,
        en: Option<&str>,
        de: Option<&str>,
    ) -> (ConsentContent, usize) {
        static NO_SOURCES: LazyLock<&'static VarsConsent> = LazyLock::new(|| vars(Vec::default()));

        let calls = AtomicUsize::new(0);
        let client = Client {
            id: "client".to_string(),
            consent: Some("https://consent.example.com/client".to_string()),
            ..Default::default()
        };
        let find = async |_url: &Url, lang: Language| {
            calls.fetch_add(1, Ordering::SeqCst);
            let doc = match lang {
                Language::En => en,
                Language::De => de,
                lang => panic!("unexpected fetch for {lang:?}"),
            };
            match doc {
                Some(doc) => RemoteConsent::parse(doc.as_bytes()),
                None => Err(ErrorResponse::new(
                    ErrorResponseType::Connection,
                    "unavailable",
                )),
            }
        };
        let content = ConsentContent::resolve_with(
            &client,
            None,
            &strs(&["data.read"]),
            lang,
            *NO_SOURCES,
            find,
        )
        .await
        .unwrap();
        (content, calls.load(Ordering::SeqCst))
    }

    /// One lookup for `en`, two for any other language (the required `en` one as well), also
    /// when the `en` document is unavailable.
    #[tokio::test]
    async fn remote_lookups_per_language() {
        for en in [Some(REMOTE), None] {
            assert_eq!(resolve_mock_counted(Language::En, en, None).await.1, 1);
            assert_eq!(
                resolve_mock_counted(Language::De, en, Some(REMOTE_DE))
                    .await
                    .1,
                2
            );
        }
    }

    /// The hash is the one of the content shown, in the user's language.
    #[tokio::test]
    async fn content_hash_is_from_the_shown_document() {
        let en = resolve_mock(Language::En, Some(REMOTE), Some(REMOTE_DE)).await;
        assert_eq!(en.source, ConsentSourceType::Remote);
        assert_eq!(en.name.as_deref(), Some("Example API"));
        let de = resolve_mock(Language::De, Some(REMOTE), Some(REMOTE_DE)).await;
        assert_eq!(de.source, ConsentSourceType::Remote);
        assert_eq!(de.name.as_deref(), Some("Beispiel-API"));
        assert_eq!(de.scopes[0].title.as_deref(), Some("Daten lesen"));
        assert_ne!(de.content_hash, en.content_hash);

        // an edit only in `de` changes the hash for a `de` user, not for an `en` one
        let changed_de = REMOTE_DE.replace("Deine Daten lesen", "Alle deine Daten lesen");
        let de2 = resolve_mock(Language::De, Some(REMOTE), Some(&changed_de)).await;
        assert_eq!(
            de2.scopes[0].description.as_deref(),
            Some("Alle deine Daten lesen")
        );
        assert_ne!(de2.content_hash, de.content_hash);
        let en2 = resolve_mock(Language::En, Some(REMOTE), Some(&changed_de)).await;
        assert_eq!(en2.content_hash, en.content_hash);

        // an edit only in `en` changes it for an `en` user, not for a `de` one
        let changed_en = REMOTE.replace("Read your data", "Read all your data");
        let en3 = resolve_mock(Language::En, Some(&changed_en), Some(REMOTE_DE)).await;
        assert_ne!(en3.content_hash, en.content_hash);
        let de3 = resolve_mock(Language::De, Some(&changed_en), Some(REMOTE_DE)).await;
        assert_eq!(de3.content_hash, de.content_hash);
    }

    /// Every shown field is part of the hash, a scope that was not requested is not.
    #[tokio::test]
    async fn content_hash_covers_every_shown_field() {
        let base = resolve_mock(Language::En, Some(REMOTE), None).await;
        for (from, to) in [
            ("Read your data", "Read all your data"),
            ("Read data", "Read the data"),
            ("https://example.com/terms", "https://example.com/terms2"),
            ("Example wants", "Example would like"),
            (r#""name": "Example API""#, r#""name": "Other API""#),
        ] {
            let changed = REMOTE.replace(from, to);
            assert_ne!(changed, REMOTE, "{from}");
            let content = resolve_mock(Language::En, Some(&changed), None).await;
            assert_ne!(content.content_hash, base.content_hash, "{from}");
        }

        // `data.write` is not requested, so it is not shown
        for (from, to) in [
            ("Change your data", "Change all your data"),
            (r#""warning": true"#, r#""warning": false"#),
        ] {
            let changed = REMOTE.replace(from, to);
            assert_ne!(changed, REMOTE, "{from}");
            let content = resolve_mock(Language::En, Some(&changed), None).await;
            assert_eq!(content.content_hash, base.content_hash, "{from}");
        }
    }

    /// A `de` user without a `de` document is shown the `en` one, with its hash.
    #[tokio::test]
    async fn missing_translation_gets_the_en_hash() {
        let en = resolve_mock(Language::En, Some(REMOTE), None).await;
        let de = resolve_mock(Language::De, Some(REMOTE), None).await;
        assert_eq!(de.content_hash, en.content_hash);
        assert_eq!(de.scopes[0].title.as_deref(), Some("Read data"));
    }

    /// A warning shown from the `en` document is part of the hash of a translated content.
    #[tokio::test]
    async fn content_hash_covers_the_en_warning_of_a_translation() {
        let de = resolve_mock(Language::De, Some(REMOTE), Some(REMOTE_DE)).await;
        assert!(!de.scopes[0].warning);

        let warned_en = REMOTE.replace(
            r#""description": "Read your data"}"#,
            r#""description": "Read your data", "warning": true}"#,
        );
        assert_ne!(warned_en, REMOTE);
        let warned = resolve_mock(Language::De, Some(&warned_en), Some(REMOTE_DE)).await;
        assert!(warned.scopes[0].warning);
        assert_ne!(warned.content_hash, de.content_hash);
    }

    /// The registry name is shown and hashed, the remote `name` it replaces is not.
    #[tokio::test]
    async fn content_hash_covers_the_shown_name() {
        let resource = "https://api.example.com";
        let url = "https://consent.example.com/api";
        let client = Client {
            id: "client".to_string(),
            ..Default::default()
        };
        let scopes = strs(&["data.read"]);
        let resolve = async |registry: &str, remote_name: &str| {
            let vars = vars(vec![
                VarsConsentSource::new(resource.to_string(), url, registry.to_string()).unwrap(),
            ]);
            let find = async |_url: &Url, _lang: Language| {
                Ok(RemoteConsent {
                    name: Some(remote_name.to_string()),
                    ..RemoteConsent::parse(REMOTE.as_bytes()).unwrap()
                })
            };
            let content = ConsentContent::resolve_with(
                &client,
                Some(resource),
                &scopes,
                Language::En,
                vars,
                find,
            )
            .await
            .unwrap();
            assert_eq!(content.name.as_deref(), Some(registry));
            content.content_hash
        };

        let base = resolve("Registry", "Remote").await;
        assert_ne!(resolve("Other Registry", "Remote").await, base);
        assert_eq!(resolve("Registry", "Other Remote").await, base);
    }

    #[test]
    fn warning_is_kept_from_the_en_document() {
        let en = RemoteConsent::parse(REMOTE.as_bytes()).unwrap();
        // a stale translation without the `data.write` warning and without `data.read`
        let de =
            RemoteConsent::parse(br#"{"scopes": {"data.write": {"title": "Daten schreiben"}}}"#)
                .unwrap();
        let content = ConsentContent::build(
            Some(RemoteDocs::new(de, Some(en.clone()))),
            None,
            &strs(&["data.read", "data.write"]),
        );
        assert_eq!(content.scopes[0].name, "data.read");
        assert!(!content.scopes[0].warning);
        assert_eq!(content.scopes[1].name, "data.write");
        assert_eq!(content.scopes[1].title.as_deref(), Some("Daten schreiben"));
        assert!(content.scopes[1].warning);

        // a warning only in the translation is shown as well
        let de = RemoteConsent::parse(
            br#"{"scopes": {"data.read": {"title": "Daten lesen", "warning": true}}}"#,
        )
        .unwrap();
        let content = ConsentContent::build(
            Some(RemoteDocs::new(de, Some(en))),
            None,
            &strs(&["data.read"]),
        );
        assert!(content.scopes[0].warning);
    }

    #[tokio::test]
    async fn unavailable_translation_shows_en() {
        let en = resolve_mock(Language::En, Some(REMOTE), None).await;
        let de = resolve_mock(Language::De, Some(REMOTE), None).await;
        assert_eq!(de, en);
        assert_eq!(de.source, ConsentSourceType::Remote);
        assert_eq!(de.scopes[0].title.as_deref(), Some("Read data"));
    }

    #[tokio::test]
    async fn unavailable_en_document_degrades_to_local() {
        let local = ConsentContent::build(None, None, &strs(&["data.read"]));
        // even with a valid translation, the hash is never mixed
        let de = resolve_mock(Language::De, None, Some(REMOTE_DE)).await;
        assert_eq!(de, local);
        // an invalid `en` document counts as unavailable
        let de = resolve_mock(Language::De, Some("[]"), Some(REMOTE_DE)).await;
        assert_eq!(de, local);
        let en = resolve_mock(Language::En, None, None).await;
        assert_eq!(en, local);
    }

    /// In-memory [`ConsentCache`]. Reads fail from the `fail_reads_from`th one on (0-based).
    #[derive(Default)]
    struct MemCache {
        map: Mutex<HashMap<String, Option<RemoteConsent>>>,
        fail_reads_from: Option<usize>,
        reads: AtomicUsize,
        puts: AtomicUsize,
    }

    impl MemCache {
        fn value(&self, idx: &str) -> Option<Option<RemoteConsent>> {
            self.map.lock().unwrap().get(idx).cloned()
        }

        fn insert(&self, idx: &str, value: Option<RemoteConsent>) {
            self.map.lock().unwrap().insert(idx.to_string(), value);
        }
    }

    impl ConsentCache for MemCache {
        async fn get(&self, idx: &str) -> Result<Option<Option<RemoteConsent>>, ErrorResponse> {
            let n = self.reads.fetch_add(1, Ordering::SeqCst);
            if self.fail_reads_from.is_some_and(|from| n >= from) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "cache unavailable",
                ));
            }
            Ok(self.value(idx))
        }

        async fn put(
            &self,
            idx: &str,
            value: Option<&RemoteConsent>,
            _ttl: u32,
        ) -> Result<(), ErrorResponse> {
            self.puts.fetch_add(1, Ordering::SeqCst);
            self.insert(idx, value.cloned());
            Ok(())
        }
    }

    fn consent_url() -> Url {
        Url::parse("https://consent.example.com/api").unwrap()
    }

    fn remote_doc() -> RemoteConsent {
        RemoteConsent::parse(REMOTE.as_bytes()).unwrap()
    }

    fn unavailable() -> ErrorResponse {
        ErrorResponse::new(ErrorResponseType::Connection, "unavailable")
    }

    fn too_many() -> ErrorResponse {
        ErrorResponse::new(ErrorResponseType::TooManyRequests(0), "too many")
    }

    #[test]
    fn cache_value_by_result() {
        assert_eq!(
            cache_value(&Ok(remote_doc()), 3600),
            Some((Some(&remote_doc()), 3600))
        );
        assert_eq!(
            cache_value(&Err(unavailable()), 3600),
            Some((None, negative_cache_ttl(3600)))
        );
        // the fetcher's own back pressure says nothing about the source
        assert_eq!(cache_value(&Err(too_many()), 3600), None);
    }

    #[test]
    fn cache_idx_per_language() {
        let url = consent_url();
        assert_ne!(
            RemoteConsent::cache_idx(&url, Language::En),
            RemoteConsent::cache_idx(&url, Language::De)
        );
        let other = Url::parse("https://consent.example.com/other").unwrap();
        assert_ne!(
            RemoteConsent::cache_idx(&url, Language::En),
            RemoteConsent::cache_idx(&other, Language::En)
        );
    }

    #[test]
    fn keeps_cached_only_a_cached_value() {
        assert!(keeps_cached::<()>(&Ok(Some(Some(remote_doc())))));
        assert!(keeps_cached::<()>(&Ok(Some(None))));
        assert!(!keeps_cached::<()>(&Ok(None)));
        // an unreadable cache is written like an empty one
        assert!(!keeps_cached(&Err(())));
    }

    #[tokio::test]
    async fn failed_fetch_is_cached_when_the_reread_fails() {
        let url = consent_url();
        let idx = RemoteConsent::cache_idx(&url, Language::En);
        // the 1st and 2nd read miss, the re-read before the write fails
        let cache = MemCache {
            fail_reads_from: Some(2),
            ..Default::default()
        };
        let res = RemoteConsent::find_with(
            &cache,
            &InFlight::default(),
            &url,
            Language::En,
            3600,
            || async { Err(unavailable()) },
        )
        .await;
        assert!(res.is_err());
        assert_eq!(cache.value(&idx), Some(None));
    }

    #[tokio::test]
    async fn failed_fetch_keeps_a_concurrently_cached_document() {
        let url = consent_url();
        let idx = RemoteConsent::cache_idx(&url, Language::En);
        let cache = MemCache::default();
        let res = RemoteConsent::find_with(
            &cache,
            &InFlight::default(),
            &url,
            Language::En,
            3600,
            || async {
                // another lookup caches the document while this fetch fails
                cache.insert(&idx, Some(remote_doc()));
                Err(unavailable())
            },
        )
        .await;
        assert!(res.is_err());
        assert_eq!(cache.value(&idx), Some(Some(remote_doc())));
        assert_eq!(cache.puts.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn concurrent_finds_share_one_fetch() {
        let url = consent_url();
        let cache = MemCache::default();
        let in_flight = InFlight::default();
        let fetches = AtomicUsize::new(0);
        let fetch = || async {
            fetches.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(remote_doc())
        };

        let finds = (0..16)
            .map(|_| RemoteConsent::find_with(&cache, &in_flight, &url, Language::En, 3600, fetch))
            .collect::<Vec<_>>();
        for res in futures::future::join_all(finds).await {
            assert_eq!(res.unwrap(), remote_doc());
        }
        assert_eq!(fetches.load(Ordering::SeqCst), 1);
        assert_eq!(cache.puts.load(Ordering::SeqCst), 1);
        // nothing is left behind
        assert_eq!(in_flight.len(), 0);

        // another key is fetched on its own
        let (en, de) = tokio::join!(
            RemoteConsent::find_with(&cache, &in_flight, &url, Language::De, 3600, fetch),
            RemoteConsent::find_with(&cache, &in_flight, &url, Language::De, 3600, fetch),
        );
        assert!(en.is_ok() && de.is_ok());
        assert_eq!(fetches.load(Ordering::SeqCst), 2);
        assert_eq!(in_flight.len(), 0);
    }

    #[tokio::test]
    async fn cancelled_find_leaves_no_in_flight_entry() {
        let url = consent_url();
        let cache = MemCache::default();
        let in_flight = InFlight::default();

        let find = RemoteConsent::find_with(&cache, &in_flight, &url, Language::En, 3600, || {
            std::future::pending()
        });
        assert!(
            tokio::time::timeout(Duration::from_millis(20), find)
                .await
                .is_err()
        );
        assert_eq!(in_flight.len(), 0);

        // a later lookup of the same key fetches again
        let res =
            RemoteConsent::find_with(&cache, &in_flight, &url, Language::En, 3600, || async {
                Ok(remote_doc())
            })
            .await;
        assert_eq!(res.unwrap(), remote_doc());
        assert_eq!(in_flight.len(), 0);
    }

    #[tokio::test]
    async fn too_many_requests_rereads_the_cache() {
        let url = consent_url();
        let idx = RemoteConsent::cache_idx(&url, Language::En);

        // another lookup caches the document while this one is rejected
        let cache = MemCache::default();
        let res = RemoteConsent::find_with(
            &cache,
            &InFlight::default(),
            &url,
            Language::En,
            3600,
            || async {
                cache.insert(&idx, Some(remote_doc()));
                Err(too_many())
            },
        )
        .await;
        assert_eq!(res.unwrap(), remote_doc());

        // otherwise the rejection is returned and nothing is cached
        let cache = MemCache::default();
        let res = RemoteConsent::find_with(
            &cache,
            &InFlight::default(),
            &url,
            Language::En,
            3600,
            || async { Err(too_many()) },
        )
        .await;
        assert!(matches!(
            res.unwrap_err().error,
            ErrorResponseType::TooManyRequests(_)
        ));
        assert_eq!(cache.value(&idx), None);
        assert_eq!(cache.puts.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn consent_required_error_code() {
        assert_eq!(
            serde_json::to_string(&ErrorResponseType::ConsentRequired).unwrap(),
            r#""consent_required""#
        );
    }
}
