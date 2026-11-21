use crate::constants::{PEER_IP_HEADER_NAME, PROXY_MODE, TRUSTED_PROXIES};
use actix_web::HttpRequest;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::AsHeaderName;
use actix_web::http::header::HeaderMap;
use actix_web::http::header::HeaderName;
use base64::{Engine as _, engine, engine::general_purpose};
use gethostname::gethostname;
use rand::RngExt;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::fmt::Debug;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tracing::error;

const B64_URL_SAFE: engine::GeneralPurpose = general_purpose::URL_SAFE;
const B64_URL_SAFE_NO_PAD: engine::GeneralPurpose = general_purpose::URL_SAFE_NO_PAD;
const B64_STD: engine::GeneralPurpose = general_purpose::STANDARD;

#[macro_export]
macro_rules! sha256 {
    ($input:expr) => {
        ring::digest::digest(&ring::digest::SHA256, $input).as_ref()
    };
}

pub fn get_local_hostname() -> String {
    let hostname_os = gethostname();
    hostname_os
        .to_str()
        .expect("Error getting the hostname from the OS")
        .to_string()
}

// Returns an alphanumeric random String with the requested length
pub fn get_rand(count: usize) -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
}

pub fn get_rand_between(lower: u64, upper: u64) -> u64 {
    rand::rng().random_range(lower..upper)
}

#[inline(always)]
pub fn base64_encode(input: &[u8]) -> String {
    B64_STD.encode(input)
}

#[inline(always)]
pub fn base64_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_STD
        .decode(b64)
        .map_err(|_| ErrorResponse::new(ErrorResponseType::BadRequest, "B64 decoding error"))
}

#[inline(always)]
pub fn base64_decode_buf(b64: &str, buf: &mut Vec<u8>) -> Result<(), ErrorResponse> {
    B64_STD
        .decode_vec(b64, buf)
        .map_err(|_| ErrorResponse::new(ErrorResponseType::BadRequest, "B64 decoding error"))
}

// Returns the given input as a base64 URL Encoded String
#[inline(always)]
pub fn base64_url_encode(input: &[u8]) -> String {
    let b64 = B64_STD.encode(input);
    b64.chars()
        .filter_map(|c| match c {
            '=' => None,
            '+' => Some('-'),
            '/' => Some('_'),
            x => Some(x),
        })
        .collect()
}

#[inline(always)]
pub fn base64_url_no_pad_encode(input: &[u8]) -> String {
    B64_URL_SAFE_NO_PAD.encode(input)
}

#[inline(always)]
pub fn base64_url_no_pad_encode_buf(input: &[u8], buf: &mut String) {
    B64_URL_SAFE_NO_PAD.encode_string(input, buf)
}

#[inline(always)]
pub fn base64_url_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_URL_SAFE
        .decode(b64)
        .map_err(|_| ErrorResponse::new(ErrorResponseType::BadRequest, "B64 URL decoding error"))
}

#[inline(always)]
pub fn base64_url_no_pad_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_URL_SAFE_NO_PAD.decode(b64).map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "B64 URL NO PAD decoding error",
        )
    })
}

#[inline(always)]
pub fn base64_url_no_pad_decode_buf(b64: &str, buf: &mut Vec<u8>) -> Result<(), ErrorResponse> {
    B64_URL_SAFE_NO_PAD
        .decode_vec(b64, buf)
        .map_err(|_| ErrorResponse::new(ErrorResponseType::BadRequest, "B64 decoding error"))
}

#[inline(always)]
pub fn new_store_id() -> String {
    get_rand(24)
}

// 192.0.0.8 is the IPv4 dummy address, according to RFC 7600.
// On the Internet, according to IANA registry, this address cannot be
// a destination address and is never global-reachable.
// So nobody can establish a TCP connection with this as source address.
const DUMMY_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 0, 0, 8));

// A marker to be added as a app data to indicate that
// dummy address should be enabled for UNIX domain socket support
pub struct UseDummyAddress;

/// The only proxy headers that carry the real client IP. The RFC 7239 `Forwarded` header is
/// deliberately not used: actix's `ConnectionInfo::realip_remote_addr()` prefers it over
/// `X-Forwarded-For` and it can be spoofed trivially.
const X_REAL_IP: HeaderName = HeaderName::from_static("x-real-ip");
const X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");

#[inline(always)]
pub fn real_ip_from_req<T: RealIpRequest>(req: &T) -> Result<IpAddr, ErrorResponse> {
    real_ip(
        *PROXY_MODE.get().unwrap(),
        TRUSTED_PROXIES.get().unwrap(),
        PEER_IP_HEADER_NAME.get().unwrap().as_deref(),
        req.peer_ip()?,
        req.headers(),
        req.dummy_addr_enabled(),
    )
}

/// Implemented for actix's request types only, to extract the real IP in one place for handlers
/// and middlewares alike. `HttpRequest::connection_info()` hands out a `Ref` guard, so the peer
/// address is parsed inside the request type's method and returned as an owned value.
pub trait RealIpRequest {
    fn peer_ip(&self) -> Result<IpAddr, ErrorResponse>;
    fn headers(&self) -> &HeaderMap;
    fn dummy_addr_enabled(&self) -> bool;
}

impl RealIpRequest for HttpRequest {
    fn peer_ip(&self) -> Result<IpAddr, ErrorResponse> {
        parse_peer_addr(
            self.connection_info().peer_addr(),
            self.dummy_addr_enabled(),
        )
    }

    fn headers(&self) -> &HeaderMap {
        self.headers()
    }

    fn dummy_addr_enabled(&self) -> bool {
        self.app_data::<UseDummyAddress>().is_some()
    }
}

impl RealIpRequest for ServiceRequest {
    fn peer_ip(&self) -> Result<IpAddr, ErrorResponse> {
        parse_peer_addr(
            self.connection_info().peer_addr(),
            self.dummy_addr_enabled(),
        )
    }

    fn headers(&self) -> &HeaderMap {
        self.headers()
    }

    fn dummy_addr_enabled(&self) -> bool {
        self.app_data::<UseDummyAddress>().is_some()
    }
}

/// The peer address of the socket is the only source that cannot be spoofed. Every header value
/// is untrusted input, which is why no header is looked at at all without `proxy_mode`, and with
/// it, every request that does not come from a trusted proxy is rejected.
/// The real client of an `X-Forwarded-For` chain is its first value.
fn real_ip(
    proxy_mode: bool,
    trusted_proxies: &[cidr::IpCidr],
    custom_header: Option<&str>,
    peer_ip: IpAddr,
    headers: &HeaderMap,
    use_dummy_addr: bool,
) -> Result<IpAddr, ErrorResponse> {
    if !proxy_mode {
        return Ok(peer_ip);
    }
    check_trusted_proxy(&peer_ip, trusted_proxies, use_dummy_addr)?;

    let from_hdr = custom_header_ip(headers, custom_header)
        .or(ip_from_hdr(headers, &X_REAL_IP))
        .or(forwarded_for_ip(headers));

    Ok(from_hdr.unwrap_or(peer_ip))
}

fn parse_peer_addr(peer_addr: Option<&str>, use_dummy_addr: bool) -> Result<IpAddr, ErrorResponse> {
    match peer_addr {
        None => {
            if use_dummy_addr {
                Ok(DUMMY_ADDRESS)
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "No IP Addr in Connection Info - this should only happen in tests",
                ))
            }
        }
        Some(peer) => match IpAddr::from_str(peer) {
            Ok(ip) => Ok(ip),
            Err(err) => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Cannot parse peer IP address: {err}"),
            )),
        },
    }
}

#[inline(always)]
fn check_trusted_proxy(
    peer_ip: &IpAddr,
    trusted_proxies: &[cidr::IpCidr],
    use_dummy_addr: bool,
) -> Result<(), ErrorResponse> {
    if use_dummy_addr && *peer_ip == DUMMY_ADDRESS {
        return Ok(());
    }
    for cidr in trusted_proxies {
        if cidr.contains(peer_ip) {
            return Ok(());
        }
    }

    error!(
        "Invalid request from IP {} which is not a trusted proxy",
        peer_ip
    );
    Err(ErrorResponse::new(
        ErrorResponseType::BadRequest,
        "Invalid IP Address",
    ))
}

pub fn build_trusted_proxies(proxies: &[String]) -> Vec<cidr::IpCidr> {
    let mut proxies = proxies
        .iter()
        .map(|range| match cidr::IpCidr::from_str(range.trim()) {
            Ok(cidr) => cidr,
            Err(err) => {
                panic!("Cannot parse trusted proxy entry {range} to CIDR: {err}");
            }
        })
        .collect::<Vec<_>>();

    // will never increase and is static afterward
    proxies.shrink_to_fit();

    proxies
}

/// A custom header name has the highest priority because some CDNs add their own header with
/// the real client IP instead of the two standard proxy headers.
#[inline(always)]
fn custom_header_ip(headers: &HeaderMap, header_name: Option<&str>) -> Option<IpAddr> {
    header_name.and_then(|name| ip_from_hdr(headers, name))
}

#[inline(always)]
fn ip_from_hdr(headers: &HeaderMap, name: impl AsHeaderName) -> Option<IpAddr> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(ip_from_hdr_value)
}

/// The first value of an `X-Forwarded-For` chain is the real client, the rest are proxies.
#[inline(always)]
fn forwarded_for_ip(headers: &HeaderMap) -> Option<IpAddr> {
    headers
        .get(&X_FORWARDED_FOR)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .and_then(ip_from_hdr_value)
}

/// Untrusted input, so an invalid value becomes `None` and the next source is tried.
/// Accepts the plain form (`1.2.3.4`), the IPv6 form without brackets (`2001:db8::1`), and the
/// IPv6 form with brackets and an optional port (`[2001:db8::1]:4010`) that proxies like HAProxy
/// write. The port is irrelevant and therefore not validated.
#[inline]
fn ip_from_hdr_value(value: &str) -> Option<IpAddr> {
    let value = value.trim().trim_matches('"');
    let value = if let Some(brackets) = value.strip_prefix('[') {
        brackets.split_once(']')?.0
    } else if value.matches(':').count() == 1 {
        // an IPv4 with a port suffix - an IPv6 always has more than one `:`
        value.split(':').next().unwrap_or(value)
    } else {
        value
    };

    match IpAddr::from_str(value) {
        Ok(ip) => Some(ip),
        Err(err) => {
            error!("Cannot parse IP from header value: {err}");
            None
        }
    }
}

#[inline]
pub fn serialize<T>(value: &T) -> Result<Vec<u8>, ErrorResponse>
where
    T: Debug + serde::Serialize,
{
    bincode_next::serde::encode_to_vec(value, bincode_next::config::legacy()).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Cannot serialize value: {err:?}"),
        )
    })
}

#[inline]
pub fn deserialize<T>(value: &[u8]) -> Result<T, ErrorResponse>
where
    T: Debug + serde::de::DeserializeOwned,
{
    let (bytes, _) = bincode_next::serde::decode_from_slice(value, bincode_next::config::legacy())
        .map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Cannot deserialize value: {err:?}"),
            )
        })?;
    Ok(bytes)
}

/// Removes `item` from a comma-separated list, comparing whole entries only.
pub fn csv_remove(list: &str, item: &str) -> String {
    list.split(',')
        .filter(|e| e.trim() != item)
        .collect::<Vec<_>>()
        .join(",")
}

/// Renames every entry equal to `old` in a comma-separated list to `new`, comparing whole
/// entries only.
pub fn csv_rename(list: &str, old: &str, new: &str) -> String {
    list.split(',')
        .map(|e| if e.trim() == old { new } else { e })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::HeaderValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_csv_remove_rename_whole_entries() {
        assert_eq!(
            csv_remove("openid,profile,email,pro", "pro"),
            "openid,profile,email"
        );
        assert_eq!(csv_remove("x,admin2,y", "admin"), "x,admin2,y");
        assert_eq!(csv_remove("devops,dev", "dev"), "devops");
        assert_eq!(csv_remove("dev", "dev"), "");
        assert_eq!(csv_remove("", "dev"), "");
        assert_eq!(csv_remove("city_code,city", "city"), "city_code");

        assert_eq!(csv_rename("devops,dev", "dev", "qa"), "devops,qa");
        assert_eq!(
            csv_rename("openid,email,mail", "mail", "contact"),
            "openid,email,contact"
        );
        assert_eq!(
            csv_rename("given_name,family_name,name", "name", "nick"),
            "given_name,family_name,nick"
        );
        assert_eq!(csv_rename("a,b", "c", "d"), "a,b");
    }

    #[test]
    fn test_get_rand() {
        let rnd = get_rand(11);
        assert_eq!(rnd.len(), 11);

        let rnd = get_rand(0);
        assert_eq!(rnd.len(), 0);

        let rnd = get_rand(1024);
        assert_eq!(rnd.len(), 1024);
    }

    fn cidrs() -> Vec<cidr::IpCidr> {
        build_trusted_proxies(&[
            "192.168.100.0/24".to_string(),
            "192.168.0.96/28".to_string(),
            "172.16.0.1/32".to_string(),
            "10.10.10.10/31".to_string(),
        ])
    }

    fn ip(s: &str) -> IpAddr {
        IpAddr::from_str(s).unwrap()
    }

    fn headers(names: &[(&str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in names {
            map.insert(
                HeaderName::from_str(name).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
        map
    }

    /// Every combination of spoofable headers must be ignored when `proxy_mode` is off, even
    /// from an IP that is not a trusted proxy.
    #[test]
    fn real_ip_without_proxy_mode_is_the_peer_addr_only() {
        let peer = ip("192.168.99.1");
        let all = headers(&[
            ("cf-connecting-ip", "1.2.3.4"),
            ("x-real-ip", "5.6.7.8"),
            ("x-forwarded-for", "9.9.9.9, 10.10.10.10"),
            ("forwarded", "for=11.12.13.14; proto=https"),
        ]);

        let real = real_ip(false, &cidrs(), Some("cf-connecting-ip"), peer, &all, false).unwrap();
        assert_eq!(real, peer, "no header may be used without proxy_mode");
    }

    /// Custom header > `X-Real-IP` > the first `X-Forwarded-For` value, and every one of them
    /// only from a trusted proxy.
    #[test]
    fn real_ip_proxy_mode_header_priority() {
        let peer = ip("172.16.0.1");
        let all = headers(&[
            ("cf-connecting-ip", "1.2.3.4"),
            ("x-real-ip", "5.6.7.8"),
            ("x-forwarded-for", "9.9.9.9, 10.10.10.10"),
        ]);
        let real = |custom: Option<&str>, map: &HeaderMap| {
            real_ip(true, &cidrs(), custom, peer, map, false).unwrap()
        };

        assert_eq!(real(Some("cf-connecting-ip"), &all), ip("1.2.3.4"));
        assert_eq!(real(None, &all), ip("5.6.7.8"));
        assert_eq!(
            real(Some("x-missing-header"), &all),
            ip("5.6.7.8"),
            "a custom header that does not exist must fall through"
        );

        let only_xff = headers(&[("x-forwarded-for", "9.9.9.9, 10.10.10.10, 172.16.0.1")]);
        assert_eq!(real(None, &only_xff), ip("9.9.9.9"));

        let none = headers(&[("host", "localhost")]);
        assert_eq!(real(None, &none), peer, "no header -> the peer address");
    }

    /// The RFC 7239 `Forwarded` header is the spoofable one and must never decide the real IP.
    #[test]
    fn real_ip_never_uses_the_forwarded_header() {
        let peer = ip("172.16.0.1");
        let forwarded = headers(&[("forwarded", "for=9.9.9.9; proto=https")]);
        let real = real_ip(true, &cidrs(), None, peer, &forwarded, false).unwrap();
        assert_eq!(real, peer, "`Forwarded` must be ignored completely");

        let with_xff = headers(&[
            ("forwarded", "for=9.9.9.9; proto=https"),
            ("x-forwarded-for", "1.2.3.4"),
        ]);
        let real = real_ip(true, &cidrs(), None, peer, &with_xff, false).unwrap();
        assert_eq!(real, ip("1.2.3.4"));
    }

    /// A proxy header may only be trusted when the direct peer is one of the trusted proxies.
    #[test]
    fn real_ip_proxy_mode_blocks_untrusted_peers() {
        let spoofed = headers(&[("x-real-ip", "1.2.3.4"), ("x-forwarded-for", "5.6.7.8")]);
        for peer in ["192.168.99.1", "172.16.0.2", "10.10.10.12"] {
            let res = real_ip(true, &cidrs(), None, ip(peer), &spoofed, false);
            assert!(res.is_err(), "{peer} must be blocked");
        }

        let unix_socket = headers(&[("x-real-ip", "1.2.3.4")]);
        let real = real_ip(true, &cidrs(), None, DUMMY_ADDRESS, &unix_socket, true).unwrap();
        assert_eq!(real, ip("1.2.3.4"), "a UNIX socket peer is a trusted proxy");
        let res = real_ip(true, &cidrs(), None, DUMMY_ADDRESS, &unix_socket, false);
        assert!(
            res.is_err(),
            "the dummy address needs the UNIX socket marker"
        );
    }

    /// Garbage in a header is not an error, it falls through to the next source.
    #[test]
    fn real_ip_invalid_header_values_fall_through() {
        let peer = ip("172.16.0.1");
        let garbage = headers(&[
            ("cf-connecting-ip", "not-an-ip"),
            ("x-real-ip", "1.2.3.4, 5.6.7.8"),
            ("x-forwarded-for", ""),
        ]);
        let real = real_ip(
            true,
            &cidrs(),
            Some("cf-connecting-ip"),
            peer,
            &garbage,
            false,
        )
        .unwrap();
        assert_eq!(real, peer, "no valid header value -> the peer address");
    }

    #[test]
    fn ip_header_value_forms() {
        let cases = [
            ("1.2.3.4", "1.2.3.4"),
            ("  1.2.3.4  ", "1.2.3.4"),
            ("1.2.3.4:4010", "1.2.3.4"),
            ("\"1.2.3.4\"", "1.2.3.4"),
            ("2001:db8::1", "2001:db8::1"),
            ("[2001:db8::1]", "2001:db8::1"),
            ("[2001:db8::1]:4010", "2001:db8::1"),
            ("\"[2001:db8::1]:4010\"", "2001:db8::1"),
            ("::1", "::1"),
            ("[::1]", "::1"),
        ];
        for (value, expected) in cases {
            assert_eq!(
                ip_from_hdr_value(value),
                Some(ip(expected)),
                "header value {value:?}"
            );
        }

        for value in ["", " ", "1.2.3", "[::1", "2001:db8::1]:4010", "for=1.2.3.4"] {
            assert_eq!(ip_from_hdr_value(value), None, "header value {value:?}");
        }
    }

    #[test]
    fn test_trusted_proxy_check() {
        let trusted = cidrs();
        let check =
            |addr: &str, dummy: bool| check_trusted_proxy(&ip(addr), &trusted, dummy).is_ok();

        assert!(check("192.168.100.1", false));
        assert!(check("192.168.100.255", false));
        assert!(!check("192.168.99.1", false));
        assert!(!check("192.168.99.255", false));

        assert!(check("192.168.0.96", false));
        assert!(check("192.168.0.111", false));
        assert!(!check("192.168.0.95", false));
        assert!(!check("192.168.0.112", false));

        assert!(check("172.16.0.1", false));
        assert!(!check("172.16.0.2", false));

        assert!(check("10.10.10.10", false));
        assert!(check("10.10.10.11", false));
        assert!(!check("10.10.10.9", false));
        assert!(!check("10.10.10.12", false));

        assert!(check("192.0.0.8", true));
        assert!(!check("192.0.0.8", false));
        assert!(check("10.10.10.11", true));
        assert!(!check("10.10.10.9", true));
    }
}
