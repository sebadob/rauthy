// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::rauthy_config::{RauthyConfig, Vars};
use rauthy_common::constants::{APPLICATION_JSON, RAUTHY_VERSION};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use reqwest::header::CONTENT_TYPE;
use reqwest::tls;
use reqwest::{Certificate, Url};
use std::cmp::min;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::LazyLock;
use std::time::Duration;
use tokio::sync::Semaphore;
use tracing::{debug, warn};

/// TLS settings parsed once and shared by the global and the ephemeral client.
static TLS_CONFIG: LazyLock<(tls::Version, Vec<Certificate>)> = LazyLock::new(|| {
    let vars = &RauthyConfig::get().vars;

    let tls_version = match vars.http_client.min_tls.as_ref() {
        "1.3" => tls::Version::TLS_1_3,
        "1.2" => tls::Version::TLS_1_2,
        "1.1" => {
            warn!(
                r#"
    You are allowing TLS 1.1 for the global HTTP client.
    Only do this, if you know what you are doing!
    "#
            );
            tls::Version::TLS_1_1
        }
        "1.0" => {
            warn!(
                r#"
    You are allowing TLS 1.0 for the global HTTP client.
    Only do this, if you know what you are doing!
    "#
            );
            tls::Version::TLS_1_0
        }
        _ => panic!("Invalid value for HTTP_MIN_TLS, allowed: '1.3', '1.2', '1.1', '1.0'"),
    };

    let certs = match vars.http_client.root_ca_bundle.as_ref() {
        Some(bundle) => {
            let certs = Certificate::from_pem_bundle(bundle.trim().as_bytes())
                .expect("Cannot parse given HTTP_CUST_ROOT_CA_BUNDLE");
            debug!(
                "Adding {} custom Root CA certificates to HTTP Client",
                certs.len()
            );
            certs
        }
        None => Vec::new(),
    };

    (tls_version, certs)
});

/// `reqwest::ClientBuilder` with the globally configured timeouts, TLS settings and Root CA
/// bundle. Base for the global HTTP client and the ephemeral client fetcher.
pub fn http_client_builder() -> reqwest::ClientBuilder {
    let vars = &RauthyConfig::get().vars;
    let (tls_version, certs) = &*TLS_CONFIG;

    #[cfg(debug_assertions)]
    let https_only = !(vars.http_client.danger_unencrypted || vars.dev.dev_mode);
    #[cfg(not(debug_assertions))]
    let https_only = !vars.http_client.danger_unencrypted;

    let mut builder = reqwest::Client::builder()
        .connect_timeout(vars.http_client.connect_timeout)
        .timeout(vars.http_client.request_timeout)
        .pool_idle_timeout(vars.http_client.idle_timeout)
        .min_tls_version(*tls_version)
        .user_agent(format!("Rauthy Client v{RAUTHY_VERSION}"))
        .https_only(https_only)
        .danger_accept_invalid_certs(vars.http_client.danger_insecure || vars.dev.dev_mode)
        .use_rustls_tls();

    for cert in certs {
        builder = builder.add_root_certificate(cert.clone());
    }

    builder
}

/// Fetcher for documents from user-provided URLs (ephemeral / CIMD clients): no redirects,
/// no proxies, non-public addresses rejected in the resolver and for IP literals, bounded
/// concurrency. `allow_private` is the single source for both address checks.
pub(crate) struct GuardedFetcher {
    client: reqwest::Client,
    allow_private: bool,
    semaphore: Semaphore,
}

/// Maximum number of concurrent fetches of user-provided URLs.
const FETCH_PERMITS: usize = 16;
/// How long a fetch waits for a free permit before it is rejected.
const FETCH_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(2);

/// Forced at startup via [`init_ephemeral_fetcher`] so a bad config fails the boot.
static EPHEMERAL_FETCHER: LazyLock<GuardedFetcher> = LazyLock::new(|| {
    GuardedFetcher::from_vars(&RauthyConfig::get().vars)
        .expect("Cannot build HTTP client for ephemeral client lookups")
});

/// Initializes the global ephemeral fetcher. Must be called at startup.
pub fn init_ephemeral_fetcher() {
    LazyLock::force(&EPHEMERAL_FETCHER);
}

impl GuardedFetcher {
    /// Builds the fetcher from [`http_client_builder`] and the ephemeral client config.
    pub(crate) fn from_vars(vars: &Vars) -> reqwest::Result<Self> {
        Self::build(
            http_client_builder(),
            vars.ephemeral_clients.danger_allow_private_addresses,
            vars.http_client.connect_timeout,
        )
    }

    /// Same as [`Self::from_vars`] with an injected `ClientBuilder` (no config file needed).
    #[cfg(test)]
    pub(crate) fn for_tests(
        allow_private: bool,
        builder: reqwest::ClientBuilder,
    ) -> reqwest::Result<Self> {
        Self::build(builder, allow_private, Duration::from_secs(5))
    }

    fn build(
        builder: reqwest::ClientBuilder,
        allow_private: bool,
        dns_timeout: Duration,
    ) -> reqwest::Result<Self> {
        let client = builder
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .dns_resolver(GuardedResolver {
                allow_private,
                timeout: dns_timeout,
            })
            .build()?;
        Ok(Self {
            client,
            allow_private,
            semaphore: Semaphore::const_new(FETCH_PERMITS),
        })
    }

    /// Fetches a document from a user-provided URL, at most `max_size` bytes.
    /// Upstream 4xx maps to `BadRequest`; 5xx and transport failures map to `Connection`.
    pub(crate) async fn fetch_bounded(
        &self,
        url: &Url,
        max_size: usize,
    ) -> Result<Vec<u8>, ErrorResponse> {
        check_literal_host(url, self.allow_private)?;

        let too_many = || {
            warn!("Too many concurrent ephemeral client lookups, rejecting {url}");
            ErrorResponse::new(
                ErrorResponseType::TooManyRequests(chrono::Utc::now().timestamp() + 1),
                "too many concurrent ephemeral client lookups",
            )
        };
        let _permit =
            match tokio::time::timeout(FETCH_ACQUIRE_TIMEOUT, self.semaphore.acquire()).await {
                Ok(Ok(permit)) => permit,
                Ok(Err(_)) | Err(_) => return Err(too_many()),
            };

        let mut res = self
            .client
            .get(url.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await
            .map_err(|err| {
                warn!("Cannot fetch ephemeral client data from {url}: {err:?}");
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "cannot fetch ephemeral client document",
                )
            })?;

        if res.status().is_redirection() {
            warn!(
                "Ephemeral client URL {url} answered with a redirect ({})",
                res.status()
            );
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "ephemeral client URLs must not redirect",
            ));
        }
        let status = res.status();
        if !status.is_success() {
            warn!("Cannot fetch ephemeral client document from {url}: status {status}");
            return Err(if status.is_client_error() {
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "ephemeral client document not available",
                )
            } else {
                ErrorResponse::new(
                    ErrorResponseType::Connection,
                    "cannot fetch ephemeral client document",
                )
            });
        }

        let too_large = || {
            warn!("Ephemeral client document from {url} exceeds {max_size} bytes");
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("ephemeral client document exceeds {max_size} bytes"),
            )
        };
        if res
            .content_length()
            .is_some_and(|len| len > max_size as u64)
        {
            return Err(too_large());
        }

        let mut bytes =
            Vec::with_capacity(min(res.content_length().unwrap_or(4096) as usize, max_size));
        while let Some(chunk) = res.chunk().await.map_err(|err| {
            warn!("Cannot read ephemeral client data from {url}: {err:?}");
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "cannot read ephemeral client document",
            )
        })? {
            if bytes.len() + chunk.len() > max_size {
                return Err(too_large());
            }
            bytes.extend_from_slice(&chunk);
        }

        Ok(bytes)
    }
}

/// Fetches a document from a user-provided URL via the global [`GuardedFetcher`].
pub(crate) async fn fetch_bounded(url: &Url, max_size: usize) -> Result<Vec<u8>, ErrorResponse> {
    EPHEMERAL_FETCHER.fetch_bounded(url, max_size).await
}

/// Resolver that drops forbidden addresses unless `allow_private` is set. reqwest connects
/// to exactly these addresses, so there is no rebinding window between check and connect.
/// IP literals never reach the resolver; see [`check_literal_host`].
struct GuardedResolver {
    allow_private: bool,
    timeout: Duration,
}

impl Resolve for GuardedResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let allow_private = self.allow_private;
        let timeout = self.timeout;
        let host = name.as_str().to_string();

        Box::pin(async move {
            let addrs = tokio::time::timeout(timeout, tokio::net::lookup_host((host.as_str(), 0)))
                .await
                .map_err(|_| format!("DNS lookup for {host} timed out"))??;

            let mut rejected = false;
            let allowed = addrs
                .filter(|addr| {
                    if !allow_private && is_forbidden_addr(addr.ip()) {
                        warn!("Refusing ephemeral client lookup: {host} resolves to {addr}");
                        rejected = true;
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<SocketAddr>>();

            if allowed.is_empty() {
                let msg = if rejected {
                    format!("{host} resolves to non-public addresses only")
                } else {
                    format!("{host} did not resolve to any address")
                };
                return Err(msg.into());
            }

            Ok(Box::new(allowed.into_iter()) as Addrs)
        })
    }
}

/// `true` if the address must never be contacted when fetching user-provided URLs.
///
/// IPv4: 0.0.0.0/8, 127.0.0.0/8, 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 100.64.0.0/10,
/// 169.254.0.0/16, 192.0.0.0/24, 192.88.99.0/24, 192.0.2.0/24, 198.51.100.0/24,
/// 203.0.113.0/24, 198.18.0.0/15, 224.0.0.0/4, 240.0.0.0/4, 255.255.255.255
/// IPv6: ::, ::1, ff00::/8, fe80::/10, fc00::/7, fec0::/10, 100::/64, 2001::/32,
/// 2001:2::/48, 2001:10::/28, 2001:db8::/32, 64:ff9b:1::/48 (whole prefix)
/// Embedded IPv4 (::ffff:x, ::x, ::ffff:0:x, 64:ff9b::/96, 2002::/16): forbidden if the
/// inner IPv4 address is
pub(crate) fn is_forbidden_addr(addr: IpAddr) -> bool {
    match addr {
        IpAddr::V4(v4) => is_forbidden_v4(v4),
        IpAddr::V6(v6) => is_forbidden_v6(v6),
    }
}

fn is_forbidden_v4(addr: Ipv4Addr) -> bool {
    let [a, b, c, _] = addr.octets();
    addr.is_unspecified()
        || addr.is_loopback()
        || addr.is_private()
        || addr.is_link_local()
        || addr.is_multicast()
        || addr.is_broadcast()
        || addr.is_documentation()
        // "this" network 0.0.0.0/8
        || a == 0
        // CGNAT 100.64.0.0/10
        || (a == 100 && (64..=127).contains(&b))
        // IETF protocol assignments 192.0.0.0/24
        || (a == 192 && b == 0 && c == 0)
        // deprecated 6to4 relay anycast 192.88.99.0/24
        || (a == 192 && b == 88 && c == 99)
        // benchmarking 198.18.0.0/15
        || (a == 198 && (b == 18 || b == 19))
        // reserved 240.0.0.0/4
        || a >= 240
}

fn is_forbidden_v6(addr: Ipv6Addr) -> bool {
    let seg = addr.segments();
    let low_v4 = || Ipv4Addr::from(((seg[6] as u32) << 16) | seg[7] as u32);

    // embedded IPv4: NAT64 / SIIT in the last 32 bits, 6to4 in bits 16..48
    let nat64 = seg[0] == 0x64 && seg[1] == 0xff9b && seg[2..6] == [0, 0, 0, 0];
    let siit = seg[0..4] == [0, 0, 0, 0] && seg[4] == 0xffff && seg[5] == 0;
    let embedded_v4 = if nat64 || siit {
        Some(low_v4())
    } else if seg[0] == 0x2002 {
        Some(Ipv4Addr::from(((seg[1] as u32) << 16) | seg[2] as u32))
    } else {
        None
    };

    addr.is_unspecified()
        || addr.is_loopback()
        || addr.is_multicast()
        || addr.is_unicast_link_local()
        || addr.is_unique_local()
        // site-local fec0::/10 (deprecated)
        || (seg[0] & 0xffc0) == 0xfec0
        // discard-only 100::/64
        || (seg[0] == 0x100 && seg[1..4] == [0, 0, 0])
        // Teredo 2001:0::/32
        || (seg[0] == 0x2001 && seg[1] == 0)
        // benchmarking 2001:2::/48
        || (seg[0] == 0x2001 && seg[1] == 2 && seg[2] == 0)
        // ORCHID 2001:10::/28
        || (seg[0] == 0x2001 && (seg[1] & 0xfff0) == 0x10)
        // documentation 2001:db8::/32
        || (seg[0] == 0x2001 && seg[1] == 0xdb8)
        // local-use NAT64 64:ff9b:1::/48 - IPv4 position is translator-chosen
        || (seg[0] == 0x64 && seg[1] == 0xff9b && seg[2] == 1)
        // IPv4-mapped (::ffff:a.b.c.d) and IPv4-compatible (::a.b.c.d)
        || addr.to_ipv4().is_some_and(is_forbidden_v4)
        || embedded_v4.is_some_and(is_forbidden_v4)
}

/// Rejects IP-literal hosts pointing to a non-public address; names are checked by the resolver.
fn check_literal_host(url: &Url, allow_private: bool) -> Result<(), ErrorResponse> {
    let literal = match url.host_str() {
        Some(host) => {
            let bare = host
                .strip_prefix('[')
                .and_then(|h| h.strip_suffix(']'))
                .unwrap_or(host);
            match bare.parse::<IpAddr>() {
                Ok(addr) => addr,
                Err(_) if url.domain().is_some() => return Ok(()),
                Err(_) => {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "ephemeral client URL has no host",
                    ));
                }
            }
        }
        None => {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "ephemeral client URL has no host",
            ));
        }
    };

    if !allow_private && is_forbidden_addr(literal) {
        warn!("Refusing ephemeral client lookup from {url}: non-public address");
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "ephemeral client URL points to a non-public address",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rauthy_config::{Vars, validate_max_document_size};
    use actix_web::ResponseError;
    use actix_web::http::StatusCode;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn test_forbidden_addrs() {
        for s in [
            "127.0.0.1",
            "127.1.2.3",
            "0.0.0.0",
            "0.1.2.3",
            "10.0.0.1",
            "172.16.0.1",
            "172.31.255.255",
            "192.168.1.1",
            "169.254.169.254",
            "100.64.0.1",
            "100.127.255.255",
            "192.0.0.1",
            "192.0.2.1",
            "192.88.99.1",
            "198.18.0.1",
            "198.51.100.1",
            "203.0.113.1",
            "224.0.0.1",
            "240.0.0.1",
            "255.255.255.255",
            "::",
            "::1",
            "fe80::1",
            "febf::1",
            "fc00::1",
            "fd12:3456::1",
            "fec0::1",
            "ff02::1",
            "100::1",
            "2001::1",
            "2001:2::1",
            "2001:10::1",
            "2001:1f::1",
            "2001:db8::1",
            "::ffff:10.0.0.1",
            "::ffff:127.0.0.1",
            "::ffff:169.254.169.254",
            "::ffff:7f00:1",
            "::10.0.0.1",
            "::ffff:0:7f00:1",
            "::ffff:0:a9fe:a9fe",
            "64:ff9b::7f00:1",
            "64:ff9b::a00:1",
            "64:ff9b:1::a9fe:a9fe",
            "64:ff9b:1:0a00:0:1:808:808",
            "64:ff9b:1::808:808",
            "2002:7f00:1::",
            "2002:a00:1::",
        ] {
            assert!(is_forbidden_addr(ip(s)), "{s} should be rejected");
        }
    }

    #[test]
    fn test_allowed_addrs() {
        for s in [
            "1.1.1.1",
            "8.8.8.8",
            "100.63.255.255",
            "100.128.0.1",
            "172.32.0.1",
            "192.0.1.1",
            "198.17.255.255",
            "198.20.0.1",
            "fe00::1",
            "fbff::1",
            "100:0:0:1::1",
            "101::1",
            "2001:1::1",
            "2001:2:1::1",
            "2001:3::1",
            "2001:20::1",
            "2001:db9::1",
            "2606:4700:4700::1111",
            "::ffff:8.8.8.8",
            "::ffff:0:808:808",
            "64:ff9b::808:808",
            "2002:808:808::",
        ] {
            assert!(!is_forbidden_addr(ip(s)), "{s} should be allowed");
        }
    }

    #[test]
    fn test_config_defaults() {
        let vars = Vars::default();
        assert!(!vars.ephemeral_clients.danger_allow_private_addresses);
        assert_eq!(vars.ephemeral_clients.max_document_size, 65536);
    }

    #[test]
    fn test_validate_max_document_size() {
        assert!(validate_max_document_size(1023).is_err());
        assert!(validate_max_document_size(1024).is_ok());
        assert!(validate_max_document_size(65536).is_ok());
    }

    fn assert_err(
        res: Result<Vec<u8>, ErrorResponse>,
        expected: ErrorResponseType,
    ) -> ErrorResponse {
        match res {
            Err(err) => {
                assert_eq!(err.error, expected, "{err:?}");
                err
            }
            Ok(body) => panic!("expected {expected:?}, got {} bytes", body.len()),
        }
    }

    fn assert_bad_request(res: Result<Vec<u8>, ErrorResponse>) -> ErrorResponse {
        assert_err(res, ErrorResponseType::BadRequest)
    }

    #[test]
    fn test_check_literal_host() {
        for s in [
            "http://127.0.0.1:1/x",
            "http://[::1]:1/x",
            "http://169.254.169.254/x",
            "http://10.0.0.1/x",
            "http://[64:ff9b::7f00:1]/x",
            "http://[64:ff9b:1::808:808]/x",
            "http://[::ffff:0:7f00:1]/x",
        ] {
            let url = Url::parse(s).unwrap();
            let err = check_literal_host(&url, false).unwrap_err();
            assert_eq!(err.error, ErrorResponseType::BadRequest, "{s}");
            check_literal_host(&url, true).unwrap_or_else(|_| panic!("{s} with allow_private"));
        }
        for s in [
            "https://[2606:4700:4700::1111]/x",
            "https://1.1.1.1/x",
            "https://example.com/x",
        ] {
            let url = Url::parse(s).unwrap();
            check_literal_host(&url, false).unwrap_or_else(|_| panic!("{s}"));
        }
    }

    /// Plain http builder, no config file needed.
    fn test_builder(timeout: Duration) -> reqwest::ClientBuilder {
        reqwest::Client::builder().timeout(timeout)
    }

    fn test_fetcher(allow_private: bool) -> GuardedFetcher {
        GuardedFetcher::for_tests(allow_private, test_builder(Duration::from_secs(5))).unwrap()
    }

    type Hits = Arc<Mutex<HashMap<String, usize>>>;

    /// Route marker: the server reads the request and then never answers.
    const HANG: &[u8] = b"";

    /// Minimal HTTP/1.1 server: route -> raw response bytes. Returns port and hit counter.
    async fn spawn_server(routes: HashMap<&'static str, Vec<u8>>) -> (u16, Hits) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let hits: Hits = Arc::new(Mutex::new(HashMap::new()));
        let hits_srv = hits.clone();
        let routes = Arc::new(routes);

        tokio::spawn(async move {
            loop {
                let (mut socket, _) = match listener.accept().await {
                    Ok(s) => s,
                    Err(_) => return,
                };
                let hits_conn = hits_srv.clone();
                let routes = routes.clone();
                tokio::spawn(async move {
                    let mut buf = Vec::new();
                    let mut tmp = [0u8; 1024];
                    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        match socket.read(&mut tmp).await {
                            Ok(0) | Err(_) => break,
                            Ok(n) => buf.extend_from_slice(&tmp[..n]),
                        }
                    }
                    let head = String::from_utf8_lossy(&buf);
                    let path = head
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or_default()
                        .to_string();
                    *hits_conn.lock().unwrap().entry(path.clone()).or_insert(0) += 1;

                    let res = routes.get(path.as_str()).cloned().unwrap_or_else(|| {
                        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                            .to_vec()
                    });
                    if res == HANG {
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        return;
                    }
                    let _ = socket.write_all(&res).await;
                    let _ = socket.shutdown().await;
                });
            }
        });

        (port, hits)
    }

    fn response(status: &str, headers: &[(&str, &str)], body: &[u8]) -> Vec<u8> {
        let mut res = format!("HTTP/1.1 {status}\r\n");
        for (k, v) in headers {
            res.push_str(&format!("{k}: {v}\r\n"));
        }
        res.push_str("Connection: close\r\n\r\n");
        let mut bytes = res.into_bytes();
        bytes.extend_from_slice(body);
        bytes
    }

    fn with_len(status: &str, body: &[u8]) -> Vec<u8> {
        response(status, &[("Content-Length", &body.len().to_string())], body)
    }

    fn chunked(body: &[u8]) -> Vec<u8> {
        let mut encoded = Vec::new();
        for chunk in body.chunks(1000) {
            encoded.extend_from_slice(format!("{:x}\r\n", chunk.len()).as_bytes());
            encoded.extend_from_slice(chunk);
            encoded.extend_from_slice(b"\r\n");
        }
        encoded.extend_from_slice(b"0\r\n\r\n");
        response("200 OK", &[("Transfer-Encoding", "chunked")], &encoded)
    }

    fn hits(hits: &Hits, path: &str) -> usize {
        hits.lock().unwrap().get(path).copied().unwrap_or(0)
    }

    fn total_hits(hits: &Hits) -> usize {
        hits.lock().unwrap().values().sum()
    }

    #[tokio::test]
    async fn test_fetch_bounded_rejects_literals_before_connect() {
        let mut routes = HashMap::new();
        routes.insert("/ok", with_len("200 OK", b"{}"));
        let (port, hits_map) = spawn_server(routes).await;

        let fetcher = test_fetcher(false);
        for host in [
            "127.0.0.1",
            "[::1]",
            "169.254.169.254",
            "10.0.0.1",
            // non-canonical IPv4 literals, normalized by the URL parser
            "2130706433",
            "0x7f000001",
            "0177.0.0.1",
            "127.1",
            // IPv4 embedded in IPv6
            "[::ffff:7f00:1]",
            "[0:0:0:0:0:ffff:169.254.169.254]",
            "[::ffff:0:7f00:1]",
            "[64:ff9b::7f00:1]",
            "[64:ff9b:1::808:808]",
        ] {
            let url = Url::parse(&format!("http://{host}:{port}/ok")).unwrap();
            assert!(
                fetcher.fetch_bounded(&url, 1024).await.is_err(),
                "{host} should be rejected"
            );
        }
        assert_eq!(total_hits(&hits_map), 0);

        // the same literals pass with allow_private
        let fetcher = test_fetcher(true);
        let url = Url::parse(&format!("http://127.1:{port}/ok")).unwrap();
        fetcher.fetch_bounded(&url, 1024).await.unwrap();
        assert_eq!(hits(&hits_map, "/ok"), 1);
    }

    #[tokio::test]
    async fn test_fetch_bounded_local_server() {
        let max = 4096usize;
        let exact = vec![b'a'; max];
        let large = vec![b'b'; max + 1];

        let mut routes = HashMap::new();
        routes.insert("/ok", with_len("200 OK", b"{\"hello\":\"world\"}"));
        routes.insert(
            "/redirect",
            response(
                "302 Found",
                &[("Location", "/target"), ("Content-Length", "0")],
                b"",
            ),
        );
        routes.insert("/target", with_len("200 OK", b"{}"));
        routes.insert("/large-len", with_len("200 OK", &large));
        routes.insert("/large-chunked", chunked(&large));
        routes.insert("/exact", with_len("200 OK", &exact));
        routes.insert("/not-found", with_len("404 Not Found", b"{}"));
        routes.insert(
            "/server-error",
            with_len("500 Internal Server Error", b"{}"),
        );
        // declares a huge body but sends 10 bytes: only the Content-Length pre-check can trip
        routes.insert(
            "/lying-len",
            response("200 OK", &[("Content-Length", "1000000")], b"0123456789"),
        );
        let (port, hits_map) = spawn_server(routes).await;

        let fetcher = test_fetcher(true);
        let url = |path: &str| Url::parse(&format!("http://127.0.0.1:{port}{path}")).unwrap();

        let body = fetcher.fetch_bounded(&url("/ok"), max).await.unwrap();
        assert_eq!(body, b"{\"hello\":\"world\"}");

        // (b) redirects are not followed
        assert_bad_request(fetcher.fetch_bounded(&url("/redirect"), max).await);
        assert_eq!(hits(&hits_map, "/redirect"), 1);
        assert_eq!(hits(&hits_map, "/target"), 0);

        // (c) too large with Content-Length
        let err = assert_bad_request(fetcher.fetch_bounded(&url("/large-len"), max).await);
        assert!(err.message.contains("exceeds"), "{err:?}");

        // (d) too large without Content-Length
        let err = assert_bad_request(fetcher.fetch_bounded(&url("/large-chunked"), max).await);
        assert!(err.message.contains("exceeds"), "{err:?}");

        // (e) exactly max_size is accepted
        let body = fetcher.fetch_bounded(&url("/exact"), max).await.unwrap();
        assert_eq!(body.len(), max);

        // (f) upstream 4xx -> BadRequest (400), upstream 5xx -> Connection (500)
        let err = assert_bad_request(fetcher.fetch_bounded(&url("/not-found"), max).await);
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "ephemeral client document not available");
        let err = assert_err(
            fetcher.fetch_bounded(&url("/server-error"), max).await,
            ErrorResponseType::Connection,
        );
        assert_eq!(err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

        // (g) lying Content-Length
        let err = assert_bad_request(fetcher.fetch_bounded(&url("/lying-len"), max).await);
        assert!(err.message.contains("exceeds"), "{err:?}");

        // (h) `localhost` resolves to loopback and is rejected by the resolver
        let fetcher = test_fetcher(false);
        let url = Url::parse(&format!("http://localhost:{port}/lh")).unwrap();
        assert_bad_request(fetcher.fetch_bounded(&url, max).await);
        assert_eq!(hits(&hits_map, "/lh"), 0);
    }

    #[tokio::test]
    async fn test_fetch_bounded_ignores_proxy() {
        let mut origin_routes = HashMap::new();
        origin_routes.insert("/ok", with_len("200 OK", b"{}"));
        let (origin_port, origin_hits) = spawn_server(origin_routes).await;
        let (proxy_port, proxy_hits) = spawn_server(HashMap::new()).await;

        let builder = test_builder(Duration::from_secs(5))
            .proxy(reqwest::Proxy::all(format!("http://127.0.0.1:{proxy_port}")).unwrap());
        let fetcher = GuardedFetcher::for_tests(true, builder).unwrap();

        let url = Url::parse(&format!("http://127.0.0.1:{origin_port}/ok")).unwrap();
        fetcher.fetch_bounded(&url, 1024).await.unwrap();
        assert_eq!(total_hits(&proxy_hits), 0);
        assert_eq!(hits(&origin_hits, "/ok"), 1);
    }

    #[tokio::test]
    async fn test_fetch_bounded_concurrency_limit() {
        let mut routes = HashMap::new();
        routes.insert("/hang", HANG.to_vec());
        routes.insert("/ok", with_len("200 OK", b"{}"));
        let (port, hits_map) = spawn_server(routes).await;

        // must outlast FETCH_ACQUIRE_TIMEOUT so the permits stay held while the extra fetch waits
        let request_timeout = FETCH_ACQUIRE_TIMEOUT + Duration::from_secs(1);
        let fetcher =
            Arc::new(GuardedFetcher::for_tests(true, test_builder(request_timeout)).unwrap());
        let hang = Url::parse(&format!("http://127.0.0.1:{port}/hang")).unwrap();

        let mut tasks = Vec::new();
        for _ in 0..FETCH_PERMITS {
            let fetcher = fetcher.clone();
            let hang = hang.clone();
            tasks.push(tokio::spawn(async move {
                fetcher.fetch_bounded(&hang, 1024).await
            }));
        }
        // wait until every permit is held
        while fetcher.semaphore.available_permits() > 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let url = Url::parse(&format!("http://127.0.0.1:{port}/ok")).unwrap();
        match fetcher.fetch_bounded(&url, 1024).await {
            Err(err) => assert!(
                matches!(err.error, ErrorResponseType::TooManyRequests(_)),
                "{err:?}"
            ),
            Ok(_) => panic!("expected TooManyRequests"),
        }
        assert_eq!(hits(&hits_map, "/ok"), 0);

        // the hanging fetches end with a request timeout and free their permits
        for task in tasks {
            assert_bad_request(task.await.unwrap());
        }
        assert_eq!(hits(&hits_map, "/hang"), FETCH_PERMITS);
        fetcher.fetch_bounded(&url, 1024).await.unwrap();
        assert_eq!(hits(&hits_map, "/ok"), 1);
    }
}
