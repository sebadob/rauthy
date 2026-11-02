// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

//! A minimal, read-only LDAPS server for clients that can only log in via an LDAP bind.
//! Binds reuse the `password` grant checks. Each client gets its own subtree, see [dn].

#![forbid(unsafe_code)]

use crate::dn::BindDn;
use futures::{SinkExt, StreamExt};
use ldap3_proto::LdapCodec;
use ldap3_proto::proto::{LdapBindResponse, LdapExtendedResponse, LdapOp, LdapResult};
use ldap3_proto::simple::{
    LdapMsg, LdapResultCode, LdapSearchScope, SearchRequest, ServerOps, SimpleBindRequest,
};
use rauthy_api_types::oidc::GrantType;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::ip_blacklist::IpBlacklist;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_service::login_delay::handle_login_delay;
use rauthy_service::password_login::password_login;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_rustls::TlsAcceptor;
use tokio_util::codec::Framed;
use tracing::{debug, error, info, warn};

mod directory;
mod dn;
mod filter;

const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const IDLE_TIMEOUT: Duration = Duration::from_secs(300);
// bound sessions are not rechecked, so this caps how long a rotated client secret stays usable
const MAX_CONN_LIFETIME: Duration = Duration::from_secs(3600);

enum Session {
    Anonymous,
    Bound {
        client_id: String,
        dn: String,
        /// `None` for a service bind, which may search the whole client subtree
        user_id: Option<String>,
    },
}

/// Runs the LDAPS listener forever. Only call it when `ldap.enable` is set.
pub async fn run(tls: Arc<rustls::ServerConfig>) {
    let vars = &RauthyConfig::get().vars.ldap;
    let addr = format!("{}:{}", vars.listen_address, vars.port);
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(err) => {
            error!("Cannot bind LDAPS listener to {addr}: {err}");
            return;
        }
    };
    info!("LDAPS listening on {addr}");

    let acceptor = TlsAcceptor::from(tls);
    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(v) => v,
            Err(err) => {
                warn!("LDAP accept error: {err}");
                continue;
            }
        };
        let acceptor = acceptor.clone();
        tokio::spawn(async move {
            let ip = peer.ip();
            if is_blacklisted(ip).await {
                return;
            }
            let stream = match timeout(TLS_HANDSHAKE_TIMEOUT, acceptor.accept(stream)).await {
                Ok(Ok(s)) => s,
                _ => {
                    debug!("LDAP TLS handshake with {ip} failed");
                    return;
                }
            };
            handle_conn(stream, ip).await;
        });
    }
}

async fn is_blacklisted(ip: IpAddr) -> bool {
    matches!(IpBlacklist::get(ip.to_string()).await, Ok(Some(_)))
}

async fn handle_conn<S: AsyncRead + AsyncWrite + Unpin>(stream: S, ip: IpAddr) {
    let mut framed = Framed::new(stream, LdapCodec::default());
    let mut session = Session::Anonymous;
    let deadline = Instant::now() + MAX_CONN_LIFETIME;

    loop {
        let wait = IDLE_TIMEOUT.min(deadline.saturating_duration_since(Instant::now()));
        let Ok(Some(Ok(msg))) = timeout(wait, framed.next()).await else {
            break;
        };
        let responses = match ServerOps::try_from(msg.clone()) {
            Ok(ServerOps::SimpleBind(req)) => {
                let (res, new_session) = bind(&req, ip).await;
                session = new_session;
                vec![res]
            }
            Ok(ServerOps::Search(req)) => search(&req, &session).await,
            // RFC 4532: an empty authzId for anonymous, `dn:<dn>` otherwise
            Ok(ServerOps::Whoami(req)) => vec![req.gen_success(&match &session {
                Session::Anonymous => String::new(),
                Session::Bound { dn, .. } => format!("dn:{dn}"),
            })],
            Ok(ServerOps::Compare(req)) => vec![req.gen_error(
                LdapResultCode::UnwillingToPerform,
                "read-only, compare is not supported".into(),
            )],
            Ok(ServerOps::Unbind(_)) => break,
            Ok(ServerOps::Abandon { .. }) => vec![],
            Err(()) => match unsupported(msg) {
                Some(res) => vec![res],
                None => break,
            },
        };

        for res in responses {
            if let Err(err) = framed.send(res).await {
                debug!("LDAP send to {ip} failed: {err}");
                return;
            }
        }
    }
}

/// Answers every operation outside the supported subset with the matching response type.
/// Returns `None` for messages a client must never send, which closes the connection.
fn unsupported(msg: LdapMsg) -> Option<LdapMsg> {
    let res = |code, message: &str| LdapResult {
        code,
        matcheddn: String::new(),
        message: message.to_string(),
        referral: Vec::new(),
    };
    let unwilling = || res(LdapResultCode::UnwillingToPerform, "read-only directory");

    let op = match msg.op {
        LdapOp::BindRequest(_) => LdapOp::BindResponse(LdapBindResponse {
            res: res(
                LdapResultCode::AuthMethodNotSupported,
                "only simple binds are supported",
            ),
            saslcreds: None,
        }),
        LdapOp::ModifyRequest(_) => LdapOp::ModifyResponse(unwilling()),
        LdapOp::AddRequest(_) => LdapOp::AddResponse(unwilling()),
        LdapOp::DelRequest(_) => LdapOp::DelResponse(unwilling()),
        LdapOp::ModifyDNRequest(_) => LdapOp::ModifyDNResponse(unwilling()),
        LdapOp::ExtendedRequest(_) => LdapOp::ExtendedResponse(LdapExtendedResponse {
            res: unwilling(),
            name: None,
            value: None,
        }),
        _ => return None,
    };
    Some(LdapMsg {
        msgid: msg.msgid,
        op,
        ctrl: Vec::new(),
    })
}

async fn bind(req: &SimpleBindRequest, ip: IpAddr) -> (LdapMsg, Session) {
    if req.dn.is_empty() && req.pw.is_empty() {
        return (req.gen_success(), Session::Anonymous);
    }
    if is_blacklisted(ip).await {
        return (
            req.gen_error(LdapResultCode::Unavailable, "IP is blacklisted".into()),
            Session::Anonymous,
        );
    }

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut user_failed_logins = None;
    let bind_dn = dn::parse_bind(&req.dn, &RauthyConfig::get().vars.ldap.base_dn);
    let is_user_bind = matches!(bind_dn, Some(BindDn::User { .. }));
    let res = check_bind(bind_dn, req.pw.clone(), ip, &mut user_failed_logins).await;
    let res = handle_login_delay(ip, start, res, is_user_bind, user_failed_logins).await;

    match res {
        Ok((client_id, user_id)) => (
            req.gen_success(),
            Session::Bound {
                client_id,
                dn: req.dn.clone(),
                user_id,
            },
        ),
        Err(err) => {
            info!(
                "LDAP bind for '{}' from {ip} failed: {}",
                req.dn, err.message
            );
            // never tell the client why, to not leak which users or clients exist
            (req.gen_invalid_cred(), Session::Anonymous)
        }
    }
}

/// Runs all bind checks and returns the bound `client_id` and, for a user bind, the user ID.
async fn check_bind(
    bind_dn: Option<BindDn>,
    password: String,
    ip: IpAddr,
    user_failed_logins: &mut Option<i64>,
) -> Result<(String, Option<String>), ErrorResponse> {
    let unauthorized = || ErrorResponse::new(ErrorResponseType::Unauthorized, "invalid bind DN");
    let (client_id, uid) = match bind_dn.ok_or_else(unauthorized)? {
        BindDn::Client(id) => (id, None),
        BindDn::User { client_id, uid } => (client_id, Some(uid)),
    };

    let client = Client::find(client_id).await?;
    client.validate_enabled()?;
    client.validate_flow(GrantType::Password)?;

    let user_id = match uid {
        None => {
            if !client.confidential {
                return Err(unauthorized());
            }
            client.check_secret(password).await?;
            None
        }
        Some(uid) => {
            let email = directory::resolve_email(&uid).await?;
            let user = password_login(&client, email, password, ip, user_failed_logins).await?;
            Some(user.id)
        }
    };

    Ok((client.id, user_id))
}

async fn search(req: &SearchRequest, session: &Session) -> Vec<LdapMsg> {
    let base_dn = &RauthyConfig::get().vars.ldap.base_dn;

    if req.base.is_empty() {
        let mut res = Vec::with_capacity(2);
        let root = directory::root_dse(base_dn);
        if req.scope == LdapSearchScope::Base && filter::matches(&req.filter, &root) {
            res.push(req.gen_result_entry(root.into_result(&req.attrs)));
        }
        res.push(req.gen_success());
        return res;
    }

    let Session::Bound {
        client_id, user_id, ..
    } = session
    else {
        return vec![req.gen_error(LdapResultCode::InsufficentAccessRights, "bind first".into())];
    };

    match search_client(req, client_id, user_id.as_deref(), base_dn).await {
        Ok(mut res) => {
            res.push(req.gen_success());
            res
        }
        Err((code, msg)) => vec![req.gen_error(code, msg)],
    }
}

/// A `user_id` limits the results to that user's own entry, which also hides it as soon as the
/// user is no longer allowed for the client.
async fn search_client(
    req: &SearchRequest,
    client_id: &str,
    user_id: Option<&str>,
    base_dn: &str,
) -> Result<Vec<LdapMsg>, (LdapResultCode, String)> {
    let client_base = dn::client_base(client_id, base_dn);
    let client_base_norm = dn::normalize(&client_base)
        .ok_or((LdapResultCode::Other, "invalid client base DN".into()))?;
    let search_base = dn::normalize(&req.base)
        .ok_or((LdapResultCode::InvalidDNSyntax, "invalid base DN".into()))?;
    if !dn::is_within(&search_base, &client_base_norm) {
        return Err((
            LdapResultCode::InsufficentAccessRights,
            format!("searches are limited to {client_base}"),
        ));
    }

    let client = Client::find(client_id.to_string())
        .await
        .and_then(|c| c.validate_enabled().map(|_| c))
        .map_err(|_| {
            (
                LdapResultCode::InsufficentAccessRights,
                "client disabled".into(),
            )
        })?;
    let (users, groups) = directory::load(&client).await.map_err(|err| {
        error!("LDAP search failed to load the directory: {}", err.message);
        (LdapResultCode::Other, "internal error".into())
    })?;

    let base_depth = dn::depth(&search_base);
    let entries = directory::build(client_id, &client_base, &users, &groups);
    // user sessions get no `noSuchObject`, it would tell them which other entries exist
    if user_id.is_none()
        && !entries
            .iter()
            .any(|e| dn::normalize(&e.dn).as_deref() == Some(search_base.as_str()))
    {
        return Err((LdapResultCode::NoSuchObject, String::new()));
    }

    Ok(entries
        .into_iter()
        .filter(|e| {
            let Some(norm) = dn::normalize(&e.dn) else {
                return false;
            };
            let in_scope = match req.scope {
                LdapSearchScope::Base => norm == search_base,
                LdapSearchScope::OneLevel => {
                    dn::is_within(&norm, &search_base) && dn::depth(&norm) == base_depth + 1
                }
                LdapSearchScope::Subtree => dn::is_within(&norm, &search_base),
                LdapSearchScope::Children => {
                    dn::is_within(&norm, &search_base) && norm != search_base
                }
            };
            in_scope
                && user_id.is_none_or(|id| e.values("entryUUID").any(|v| v == id))
                && filter::matches(&req.filter, e)
        })
        .map(|e| req.gen_result_entry(e.into_result(&req.attrs)))
        .collect())
}
