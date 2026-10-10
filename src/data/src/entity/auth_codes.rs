use crate::database::{Cache, DB};
use crate::entity::clients::Client;
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
#[cfg(debug_assertions)]
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::time::Duration;
use utoipa::ToSchema;

/// `AuthCode` as cached before `client_generation` existed.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
struct AuthCodeNoGeneration {
    id: String,
    exp: i64,
    client_id: String,
    redirect_uri: String,
    user_id: String,
    session_id: Option<String>,
    challenge: Option<String>,
    challenge_method: Option<String>,
    nonce: Option<String>,
    scopes: Vec<String>,
    resource: Option<String>,
    state: Option<Vec<u8>>,
}

#[derive(Deserialize, Serialize)]
pub struct AuthCode {
    pub id: String,
    pub exp: i64,
    pub client_id: String,
    /// The exact URI used during authorization
    pub redirect_uri: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub challenge: Option<String>,
    pub challenge_method: Option<String>,
    pub nonce: Option<String>,
    pub scopes: Vec<String>,
    /// RFC 8707 resource indicator chosen at the authorization request, carried through
    /// to the token exchange so the issued access token can be audience-restricted.
    /// No `serde` skip/default attributes here: auth codes are cached with bincode (a
    /// positional, non-self-describing format), so the field must always be present.
    pub resource: Option<String>,
    /// The opaque `state` sha256 hash from the authorization request, bound to this code so that
    /// consumers can verify a presented `state` belongs to exactly this code. No `serde`
    /// skip/default attributes here: auth codes are cached with bincode (a positional,
    /// non-self-describing format), so the field must always be present.
    ///
    /// Note: This is only used for Forward Auth, because in this case, we are also our own client.
    /// We will not save the state during normal logins, because it does not make any sense. We will
    /// not get anything from the client we could compare it to.
    pub state: Option<Vec<u8>>,
    /// `Client::generation` of `client_id` when the code was issued. Last on purpose: bincode
    /// ignores trailing bytes, so older versions can still decode a code with this field.
    pub client_generation: String,
}

impl Debug for AuthCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthCode {{ id: {}(...), exp: {}, client_id: {}, user_id: {}, scopes: {:?}, state: {:?} }}",
            &self.id[..5],
            self.exp,
            self.client_id,
            self.user_id,
            self.scopes,
            self.state,
        )
    }
}

// CRUD
impl AuthCode {
    // Deletes an Authorization Code from the cache
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::AuthCode, self.id.clone()).await?;
        Ok(())
    }

    // Claims an Authorization code from the cache
    pub async fn find_remove(id: String) -> Result<Option<Self>, ErrorResponse> {
        let Some(bytes) = DB::hql().get_remove_bytes(Cache::AuthCode, id).await? else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "`auth_code` not found",
            ));
        };

        Self::decode(&bytes)
            .map(Some)
            .ok_or_else(|| ErrorResponse::new(ErrorResponseType::NotFound, "auth_code not found"))

        // Ok(DB::hql().get_remove(Cache::AuthCode, id).await?)
    }

    /// Decodes a cached code, falling back to the layout without `client_generation`. Such codes
    /// get the empty one, which every client has until it is recreated.
    fn decode(bytes: &[u8]) -> Option<Self> {
        #[cfg(debug_assertions)]
        if !RAUTHY_VERSION.starts_with("0.37.") && !RAUTHY_VERSION.starts_with("0.38.") {
            todo!("Remove AuthCodeNoGeneration");
        }

        let config = bincode_next::config::legacy();
        if let Ok((slf, _)) = bincode_next::serde::decode_from_slice::<Self, _>(bytes, config) {
            return Some(slf);
        }

        let (code, _) =
            bincode_next::serde::decode_from_slice::<AuthCodeNoGeneration, _>(bytes, config)
                .ok()?;
        Some(Self {
            id: code.id,
            exp: code.exp,
            client_id: code.client_id,
            redirect_uri: code.redirect_uri,
            user_id: code.user_id,
            session_id: code.session_id,
            challenge: code.challenge,
            challenge_method: code.challenge_method,
            nonce: code.nonce,
            scopes: code.scopes,
            resource: code.resource,
            state: code.state,
            client_generation: String::new(),
        })
    }

    // Saves an Authorization Code
    pub async fn save(&self, ttl: i64) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(Cache::AuthCode, self.id.clone(), self, Some(ttl))
            .await?;
        Ok(())
    }
}

/// Appends `params`, `state` and the RFC 9207 `iss`, form-urlencoded, to `redirect_uri`.
/// `redirect_uri` must already have passed `validate_redirect_uri_shape`.
#[must_use]
pub fn authorization_redirect(
    redirect_uri: &str,
    params: &[(&str, &str)],
    state: Option<&str>,
    issuer: &str,
) -> String {
    let state = state.map(|state| ("state", state));
    let pairs = params.iter().copied().chain(state).chain([("iss", issuer)]);
    append_query(redirect_uri, pairs)
}

/// Appends `state`, form-urlencoded, to `post_logout_redirect_uri`, or returns it unchanged
/// without a `state`. `post_logout_redirect_uri` must already have passed
/// `validate_post_logout_redirect_uri_shape`.
#[must_use]
pub fn post_logout_redirect(post_logout_redirect_uri: &str, state: Option<&str>) -> String {
    match state {
        Some(state) => append_query(post_logout_redirect_uri, [("state", state)]),
        None => post_logout_redirect_uri.to_string(),
    }
}

/// Appends `pairs`, form-urlencoded, to the query of `uri`, which must not contain a fragment.
///
/// A space is sent as `%20` rather than the form encoding `+`, as in previous releases, so that
/// clients decoding the query as plain percent-encoding get the original value back. This is
/// safe because the serializer encodes a literal `+` as `%2B`.
fn append_query<'a>(uri: &str, pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> String {
    debug_assert!(!uri.contains('#'));
    let append_char = if uri.contains('?') { '&' } else { '?' };

    let mut query = form_urlencoded::Serializer::new(String::with_capacity(128));
    for (key, value) in pairs {
        query.append_pair(key, value);
    }
    let query = query.finish().replace('+', "%20");

    let mut loc = String::with_capacity(uri.len() + 1 + query.len());
    loc.push_str(uri);
    loc.push(append_char);
    loc.push_str(&query);
    loc
}

impl AuthCode {
    #[inline]
    #[must_use]
    pub fn build_location_header(&self, state: Option<&str>) -> String {
        authorization_redirect(
            &self.redirect_uri,
            &[("code", &self.id)],
            state,
            &RauthyConfig::get().issuer,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: String,
        client_id: String,
        client_generation: String,
        redirect_uri: String,
        session_id: Option<String>,
        challenge: Option<String>,
        challenge_method: Option<String>,
        nonce: Option<String>,
        scopes: Vec<String>,
        resource: Option<String>,
        state: Option<Vec<u8>>,
        lifetime: Duration,
    ) -> Self {
        debug_assert!(!redirect_uri.is_empty());
        debug_assert!(lifetime.as_secs() > 0);

        let id = get_rand(64);
        let exp = Utc::now().add(lifetime).timestamp();

        Self {
            id,
            exp,
            client_id,
            client_generation,
            redirect_uri,
            user_id,
            session_id,
            challenge,
            challenge_method,
            nonce,
            scopes,
            resource,
            state,
        }
    }

    /// `current` is the stored `Client::generation` of `client_id`, `None` if it does not exist.
    #[inline]
    pub fn is_for_client_generation(&self, current: Option<&str>) -> bool {
        current == Some(self.client_generation.as_str())
    }

    /// CAUTION: DO NOT use this reset in any other case than after accepting updated ToS!
    pub async fn danger_save_reset_exp(
        &mut self,
        auth_code_lifetime: i64,
    ) -> Result<(), ErrorResponse> {
        self.exp = Utc::now()
            .add(chrono::Duration::seconds(auth_code_lifetime))
            .timestamp();

        self.save(auth_code_lifetime).await
    }

    #[inline(always)]
    pub fn validate_redirect_uri_exact(
        &self,
        client: &Client,
        redirect_uri: &str,
        rfc_8252_enable: bool,
    ) -> Result<(), ErrorResponse> {
        // The `client.validate_redirect_uri()` already prevents an empty URI, this makes it obvious.
        debug_assert!(!self.redirect_uri.is_empty());

        // Technically, this additional validation is not necessary, but it does not hurt either,
        // and it just an additional defense. It's a very inexpensive operation.
        client.validate_redirect_uri_with(redirect_uri, rfc_8252_enable)?;

        if self.redirect_uri == redirect_uri {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Invalid `redirect_uri`",
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct AuthCodeToSAwait {
    pub auth_code: String,
    pub await_code: String,
    pub auth_code_lifetime: i32,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub needs_user_update: bool,
}

// CRUD
impl AuthCodeToSAwait {
    pub async fn find_remove(code: &str) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::hql()
            .get_remove(Cache::AuthCode, Self::cache_idx(code))
            .await?)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(
                Cache::AuthCode,
                Self::cache_idx(&self.await_code),
                &self,
                Some(RauthyConfig::get().vars.tos.accept_timeout.as_secs() as i64),
            )
            .await?;

        Ok(())
    }
}

impl AuthCodeToSAwait {
    #[inline]
    fn cache_idx(await_code: &str) -> String {
        // Note: We don't want to simply index the await
        // codes by id to never have an accidental misuse.
        format!("tos_aw_{await_code}")
    }

    #[inline]
    pub fn generate_code() -> String {
        get_rand(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISSUER: &str = "https://iam.example.com/auth/v1/";
    const ISSUER_ENC: &str = "https%3A%2F%2Fiam.example.com%2Fauth%2Fv1%2F";
    const CB: &str = "https://client.example.com/cb";

    fn decoded_query(loc: &str) -> Vec<(String, String)> {
        let url = reqwest::Url::parse(loc).expect("location to parse");
        assert!(url.fragment().is_none(), "{loc}");
        url.query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect()
    }

    #[test]
    fn test_success_without_state() {
        let loc = authorization_redirect(CB, &[("code", "c0de")], None, ISSUER);
        assert_eq!(loc, format!("{CB}?code=c0de&iss={ISSUER_ENC}"));
        assert_eq!(
            decoded_query(&loc),
            vec![
                ("code".to_string(), "c0de".to_string()),
                ("iss".to_string(), ISSUER.to_string()),
            ]
        );
    }

    #[test]
    fn test_success_with_state_and_existing_query() {
        let loc = authorization_redirect(
            &format!("{CB}?foo=bar"),
            &[("code", "c0de")],
            Some("st4te"),
            ISSUER,
        );
        assert_eq!(
            loc,
            format!("{CB}?foo=bar&code=c0de&state=st4te&iss={ISSUER_ENC}")
        );
        assert_eq!(
            decoded_query(&loc),
            vec![
                ("foo".to_string(), "bar".to_string()),
                ("code".to_string(), "c0de".to_string()),
                ("state".to_string(), "st4te".to_string()),
                ("iss".to_string(), ISSUER.to_string()),
            ]
        );
    }

    #[test]
    fn test_state_cannot_inject_iss() {
        let state = "orig&iss=https://attacker.example.com/";
        let loc = authorization_redirect(CB, &[("code", "c0de")], Some(state), ISSUER);

        assert_eq!(loc.matches("iss=").count(), 1);
        assert!(loc.ends_with(&format!("&iss={ISSUER_ENC}")));
        assert!(!loc.contains("attacker.example.com/"));

        assert_eq!(
            decoded_query(&loc),
            vec![
                ("code".to_string(), "c0de".to_string()),
                ("state".to_string(), state.to_string()),
                ("iss".to_string(), ISSUER.to_string()),
            ]
        );
    }

    #[test]
    fn test_state_space_and_plus() {
        let state = "a b+c";
        for params in [[("code", "c0de")], [("error", "login_required")]] {
            let loc = authorization_redirect(CB, &params, Some(state), ISSUER);
            let (key, value) = params[0];
            assert_eq!(
                loc,
                format!("{CB}?{key}={value}&state=a%20b%2Bc&iss={ISSUER_ENC}")
            );

            // a form decoder recovers the original value
            assert_eq!(
                decoded_query(&loc),
                vec![
                    (key.to_string(), value.to_string()),
                    ("state".to_string(), state.to_string()),
                    ("iss".to_string(), ISSUER.to_string()),
                ]
            );

            // and so does a plain percent decoder
            let raw = loc
                .split('&')
                .find_map(|pair| pair.strip_prefix("state="))
                .unwrap();
            assert_eq!(
                percent_encoding::percent_decode_str(raw)
                    .decode_utf8()
                    .unwrap(),
                state
            );
        }
    }

    #[test]
    fn test_validate_redirect_uri_exact_rejects_stored_fragment_uri() {
        // a URI stored before fragments were rejected at registration time
        let uri = "https://app.example.com/#/cb";
        let client = crate::entity::clients::tests::redirect_test_client("legacy", uri);
        let code = AuthCode {
            id: "c0de".to_string(),
            exp: 0,
            client_id: client.id.clone(),
            client_generation: client.generation.clone(),
            redirect_uri: uri.to_string(),
            user_id: "user".to_string(),
            session_id: None,
            challenge: None,
            challenge_method: None,
            nonce: None,
            scopes: vec!["openid".to_string()],
            resource: None,
            state: None,
        };

        let err = code
            .validate_redirect_uri_exact(&client, uri, false)
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::BadRequest);
        assert_eq!(err.message, "`redirect_uri` must not contain any of: # ,");
    }

    #[test]
    fn test_post_logout_redirect() {
        const BYE: &str = "https://client.example.com/bye";

        // without a `state`, the URI is used as it is
        assert_eq!(post_logout_redirect(BYE, None), BYE);
        let with_query = format!("{BYE}?foo=bar");
        assert_eq!(post_logout_redirect(&with_query, None), with_query);

        let loc = post_logout_redirect(BYE, Some("st4te"));
        assert_eq!(loc, format!("{BYE}?state=st4te"));

        // an existing query is kept, and `state` is appended exactly once
        let loc = post_logout_redirect(&with_query, Some("st4te"));
        assert_eq!(loc, format!("{BYE}?foo=bar&state=st4te"));
        assert_eq!(
            decoded_query(&loc),
            vec![
                ("foo".to_string(), "bar".to_string()),
                ("state".to_string(), "st4te".to_string()),
            ]
        );

        // a space is sent as `%20`, a `+` as `%2B`
        let state = "a b+c";
        let loc = post_logout_redirect(BYE, Some(state));
        assert_eq!(loc, format!("{BYE}?state=a%20b%2Bc"));
        assert_eq!(
            decoded_query(&loc),
            vec![("state".to_string(), state.to_string())]
        );
        let raw = loc.split_once("state=").unwrap().1;
        assert_eq!(
            percent_encoding::percent_decode_str(raw)
                .decode_utf8()
                .unwrap(),
            state
        );

        // a `state` cannot add another parameter or a fragment
        let state = "x&state=evil#frag";
        let loc = post_logout_redirect(&with_query, Some(state));
        assert_eq!(loc.matches("state=").count(), 1, "{loc}");
        assert_eq!(
            decoded_query(&loc),
            vec![
                ("foo".to_string(), "bar".to_string()),
                ("state".to_string(), state.to_string()),
            ]
        );
    }

    #[test]
    fn test_error_redirect() {
        let loc = authorization_redirect(CB, &[("error", "login_required")], Some("x"), ISSUER);
        assert_eq!(
            loc,
            format!("{CB}?error=login_required&state=x&iss={ISSUER_ENC}")
        );
        assert_eq!(
            decoded_query(&loc),
            vec![
                ("error".to_string(), "login_required".to_string()),
                ("state".to_string(), "x".to_string()),
                ("iss".to_string(), ISSUER.to_string()),
            ]
        );
    }

    #[test]
    fn test_auth_code_client_generation() {
        let code = AuthCode {
            id: "c0de".to_string(),
            exp: 0,
            client_id: "client_1".to_string(),
            client_generation: "gen_a".to_string(),
            redirect_uri: "https://client.example.com/cb".to_string(),
            user_id: "user_1".to_string(),
            session_id: None,
            challenge: None,
            challenge_method: None,
            nonce: None,
            scopes: Vec::new(),
            resource: None,
            state: None,
        };

        assert!(code.is_for_client_generation(Some("gen_a")));
        assert!(!code.is_for_client_generation(Some("gen_b")));
        assert!(!code.is_for_client_generation(Some("")));
        assert!(!code.is_for_client_generation(None));
    }

    fn encode<T: Serialize>(value: &T) -> Vec<u8> {
        bincode_next::serde::encode_to_vec(value, bincode_next::config::legacy()).unwrap()
    }

    fn code_with_generation(generation: &str) -> AuthCode {
        AuthCode {
            id: "c0de".to_string(),
            exp: 1,
            client_id: "client_1".to_string(),
            redirect_uri: "https://client.example.com/cb".to_string(),
            user_id: "user_1".to_string(),
            session_id: Some("sid".to_string()),
            challenge: None,
            challenge_method: None,
            nonce: None,
            scopes: vec!["openid".to_string()],
            resource: None,
            state: Some(vec![1, 2]),
            client_generation: generation.to_string(),
        }
    }

    #[test]
    fn test_auth_code_decode_current() {
        let code = AuthCode::decode(&encode(&code_with_generation("gen_a"))).unwrap();
        assert_eq!(code.client_generation, "gen_a");
        assert_eq!(code.redirect_uri, "https://client.example.com/cb");
        assert_eq!(code.state, Some(vec![1, 2]));
    }

    #[test]
    fn test_auth_code_decode_without_generation() {
        let old = AuthCodeNoGeneration {
            id: "c0de".to_string(),
            exp: 1,
            client_id: "client_1".to_string(),
            redirect_uri: "https://client.example.com/cb".to_string(),
            user_id: "user_1".to_string(),
            session_id: Some("sid".to_string()),
            challenge: None,
            challenge_method: None,
            nonce: None,
            scopes: vec!["openid".to_string()],
            resource: None,
            state: Some(vec![1, 2]),
        };
        let code = AuthCode::decode(&encode(&old)).unwrap();
        assert_eq!(code.client_generation, "");
        assert_eq!(code.redirect_uri, "https://client.example.com/cb");
        assert_eq!(code.user_id, "user_1");
        assert_eq!(code.state, Some(vec![1, 2]));
        assert!(code.is_for_client_generation(Some("")));
    }

    #[test]
    fn test_auth_code_readable_by_older_versions() {
        let bytes = encode(&code_with_generation("gen_a"));
        let (old, _) = bincode_next::serde::decode_from_slice::<AuthCodeNoGeneration, _>(
            &bytes,
            bincode_next::config::legacy(),
        )
        .unwrap();
        assert_eq!(old.redirect_uri, "https://client.example.com/cb");
        assert_eq!(old.state, Some(vec![1, 2]));
    }
}
