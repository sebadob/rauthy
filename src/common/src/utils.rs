use crate::constants::{PEER_IP_HEADER_NAME, PROXY_MODE, TRUSTED_PROXIES};
use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderMap;
use actix_web::HttpRequest;
use base64::{engine, engine::general_purpose, Engine as _};
use gethostname::gethostname;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::net::IpAddr;
use std::str::FromStr;
use std::{env, net::Ipv4Addr};
use tracing::{error, trace};

const B64_URL_SAFE: engine::GeneralPurpose = general_purpose::URL_SAFE;
const B64_URL_SAFE_NO_PAD: engine::GeneralPurpose = general_purpose::URL_SAFE_NO_PAD;
const B64_STD: engine::GeneralPurpose = general_purpose::STANDARD;

pub fn get_local_hostname() -> String {
    let hostname_os = gethostname();
    hostname_os
        .to_str()
        .expect("Error getting the hostname from the OS")
        .to_string()
}

// Returns an alphanumeric random String with the requested length
pub fn get_rand(count: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
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
pub fn base64_url_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_URL_SAFE
        .decode(b64)
        .map_err(|_| ErrorResponse::new(ErrorResponseType::BadRequest, "B64 decoding error"))
}

#[inline(always)]
pub fn base64_url_no_pad_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_URL_SAFE_NO_PAD
        .decode(b64)
        .map_err(|_| ErrorResponse::new(ErrorResponseType::BadRequest, "B64 decoding error"))
}

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

// TODO unify real_ip_from_req and real_ip_from_svc_req by using an impl Trait
#[inline(always)]
pub fn real_ip_from_req(req: &HttpRequest) -> Result<IpAddr, ErrorResponse> {
    let use_dummy_addr = req.app_data::<UseDummyAddress>().is_some();
    let peer_ip = parse_peer_addr(req.connection_info().peer_addr(), use_dummy_addr)?;
    if let Some(ip) = ip_from_cust_header(req.headers()) {
        check_trusted_proxy(&peer_ip, use_dummy_addr)?;
        Ok(ip)
    } else if *PROXY_MODE {
        check_trusted_proxy(&peer_ip, use_dummy_addr)?;
        parse_peer_addr(req.connection_info().realip_remote_addr(), false)
    } else {
        Ok(peer_ip)
    }
}

#[inline(always)]
pub fn real_ip_from_svc_req(req: &ServiceRequest) -> Result<IpAddr, ErrorResponse> {
    let use_dummy_addr = req.app_data::<UseDummyAddress>().is_some();
    let peer_ip = parse_peer_addr(req.connection_info().peer_addr(), use_dummy_addr)?;
    if let Some(ip) = ip_from_cust_header(req.headers()) {
        check_trusted_proxy(&peer_ip, use_dummy_addr)?;
        Ok(ip)
    } else if *PROXY_MODE {
        check_trusted_proxy(&peer_ip, use_dummy_addr)?;
        parse_peer_addr(req.connection_info().realip_remote_addr(), false)
    } else {
        Ok(peer_ip)
    }
}

#[inline(always)]
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
                format!("Cannot parse peer IP address: {}", err),
            )),
        },
    }
}

#[inline(always)]
fn check_trusted_proxy(peer_ip: &IpAddr, use_dummy_addr: bool) -> Result<(), ErrorResponse> {
    if use_dummy_addr && *peer_ip == DUMMY_ADDRESS {
        return Ok(());
    }
    for cidr in &*TRUSTED_PROXIES {
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

pub(crate) fn build_trusted_proxies() -> Vec<cidr::IpCidr> {
    let raw = env::var("TRUSTED_PROXIES").expect("TRUSTED_PROXIES is not net set");
    let mut proxies = Vec::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match cidr::IpCidr::from_str(trimmed) {
            Ok(cidr) => {
                proxies.push(cidr);
            }
            Err(err) => {
                error!("Cannot parse trusted proxy entry to CIDR: {}", err);
            }
        }
    }
    proxies
}

#[inline(always)]
fn ip_from_cust_header(headers: &HeaderMap) -> Option<IpAddr> {
    // If a custom override has been set, try this first and use the default as fallback
    if let Some(header_name) = &*PEER_IP_HEADER_NAME {
        if let Some(Ok(value)) = headers.get(header_name).map(|s| s.to_str()) {
            match IpAddr::from_str(value) {
                Ok(ip) => {
                    return Some(ip);
                }
                Err(err) => {
                    error!("Cannot parse IP from PEER_IP_HEADER_NAME: {}", err);
                }
            }
        }
        trace!("no PEER IP from PEER_IP_HEADER_NAME: '{}'", header_name);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_rand() {
        let rnd = get_rand(11);
        assert_eq!(rnd.len(), 11);

        let rnd = get_rand(0);
        assert_eq!(rnd.len(), 0);

        let rnd = get_rand(1024);
        assert_eq!(rnd.len(), 1024);
    }

    #[test]
    fn test_trusted_proxy_check() {
        env::set_var(
            "TRUSTED_PROXIES",
            r#"
            192.168.100.0/24
            192.168.0.96/28
            172.16.0.1/32
            10.10.10.10/31"#,
        );

        println!("{:?}", build_trusted_proxies());

        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.100.1").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.100.255").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.99.1").unwrap(), false).is_err());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.99.255").unwrap(), false).is_err());

        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.0.96").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.0.111").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.0.95").unwrap(), false).is_err());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.168.0.112").unwrap(), false).is_err());

        assert!(check_trusted_proxy(&IpAddr::from_str("172.16.0.1").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("172.16.0.2").unwrap(), false).is_err());

        assert!(check_trusted_proxy(&IpAddr::from_str("10.10.10.10").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("10.10.10.11").unwrap(), false).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("10.10.10.9").unwrap(), false).is_err());
        assert!(check_trusted_proxy(&IpAddr::from_str("10.10.10.12").unwrap(), false).is_err());

        assert!(check_trusted_proxy(&IpAddr::from_str("192.0.0.8").unwrap(), true).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("192.0.0.8").unwrap(), false).is_err());
        assert!(check_trusted_proxy(&IpAddr::from_str("10.10.10.11").unwrap(), true).is_ok());
        assert!(check_trusted_proxy(&IpAddr::from_str("10.10.10.9").unwrap(), true).is_err());
    }
}
