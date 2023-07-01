// use crate::constants::PROXY_MODE;
// use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
// use actix_web::dev::ServiceRequest;
// use serde::{Deserialize, Serialize};
// use std::net::{IpAddr, SocketAddr};
// use std::str::FromStr;
// use tracing::debug;
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct RealIpKeyExtractor;
//
// impl KeyExtractor for RealIpKeyExtractor {
//     type Key = IpAddr;
//     type KeyExtractionError = SimpleKeyExtractionError<&'static str>;
//
//     fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
//         let connection_info = req.connection_info();
//         if *PROXY_MODE {
//             // The request is coming from the reverse proxy, we can trust the `Forwarded` or `X-Forwarded-For` headers
//             let ip = connection_info.realip_remote_addr();
//             debug!("RealIpKeyExtractor - remote IP: {:?}", ip);
//
//             if let Some(peer) = ip {
//                 SocketAddr::from_str(peer)
//                     .map(|socket| socket.ip())
//                     .or_else(|_| IpAddr::from_str(peer))
//                     .map_err(|_| {
//                         SimpleKeyExtractionError::new(
//                             "Could not extract real IP address from request",
//                         )
//                     })
//             } else {
//                 Err(SimpleKeyExtractionError::new(
//                     "Could not extract the real IP address from the request",
//                 ))
//             }
//         } else {
//             // The request is not coming from the reverse proxy, we use peer IP
//             let ip = connection_info.peer_addr();
//             debug!("RealIpKeyExtractor - remote IP: {:?}", ip);
//
//             ip.ok_or_else(|| {
//                 SimpleKeyExtractionError::new("Could not extract peer IP address from request")
//             })
//             .and_then(|str| {
//                 SocketAddr::from_str(str).map_err(|_| {
//                     SimpleKeyExtractionError::new("Could not extract peer IP address from request")
//                 })
//             })
//             .map(|socket| socket.ip())
//         }
//     }
// }
//
// #[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
// pub struct AuthHeaderExtractor;
//
// impl KeyExtractor for AuthHeaderExtractor {
//     type Key = String;
//     type KeyExtractionError = SimpleKeyExtractionError<&'static str>;
//
//     fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
//         req.headers()
//             .get("Authorization")
//             .and_then(|token| token.to_str().ok())
//             .and_then(|token| token.strip_prefix("Bearer "))
//             .map(|token| token.trim().to_owned())
//             .ok_or_else(|| SimpleKeyExtractionError::new("You don't have permission to access"))
//     }
// }
