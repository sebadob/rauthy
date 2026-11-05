// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

//! A minimal, read-only LDAPS server for clients that can only log in via an LDAP bind.
//! Binds reuse the `password` grant checks. Each client gets its own subtree, see [dn].

#![forbid(unsafe_code)]

use crate::directory::{Entry, Target};
use crate::dn::BindDn;
use futures::{SinkExt, StreamExt};
use ldap3_proto::LdapCodec;
use ldap3_proto::proto::{LdapBindResponse, LdapExtendedResponse, LdapOp, LdapResult};
use ldap3_proto::simple::{
    LdapFilter, LdapMsg, LdapResultCode, LdapSearchScope, SearchRequest, ServerOps,
    SimpleBindRequest,
};
use rauthy_api_types::oidc::GrantType;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::ip_blacklist::IpBlacklist;
use rauthy_data::entity::users::User;
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
// each targeted user costs several sequential DB queries, so a search must stay far below a full load
const MAX_TARGETS: usize = 50;

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
    // all users of an app bind from the app's IP, so the per-IP checks can be disabled for them
    let ip_checks = !(is_user_bind
        && RauthyConfig::get()
            .vars
            .ldap
            .disable_ip_brute_force_protection);
    let res = check_bind(
        bind_dn,
        req.pw.clone(),
        ip,
        &mut user_failed_logins,
        ip_checks,
    )
    .await;
    let res = if res.is_ok() && !is_user_bind {
        // a successful service bind must not reset the per-IP counter of the user binds after it
        res
    } else {
        handle_login_delay(ip, start, res, is_user_bind, user_failed_logins, ip_checks).await
    };

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
    ip_checks: bool,
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
            let user =
                password_login(&client, email, password, ip, user_failed_logins, ip_checks).await?;
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
    let search_base = search_base(&req.base, &client_base)?;

    let client = Client::find(client_id.to_string())
        .await
        .and_then(|c| c.validate_enabled().map(|_| c))
        .map_err(|_| {
            (
                LdapResultCode::InsufficentAccessRights,
                "client disabled".into(),
            )
        })?;
    let internal = |err: ErrorResponse| {
        error!("LDAP search failed to load the directory: {}", err.message);
        (LdapResultCode::Other, "internal error".to_string())
    };
    let user_count = User::count().await.map_err(internal)?;
    let threshold = RauthyConfig::get().vars.server.ssp_threshold;
    let targets = targets(user_count, threshold, user_id, &search_base, &req.filter)?;
    let (users, groups) = directory::load(&client, targets).await.map_err(internal)?;

    let entries = directory::build(client_id, &client_base, &users, &groups);
    Ok(visible(
        entries,
        &search_base,
        req.scope.clone(),
        user_id,
        &req.filter,
    )?
    .into_iter()
    .map(|e| req.gen_result_entry(e.into_result(&req.attrs)))
    .collect())
}

/// Below `server.ssp_threshold` users, searches load the whole directory (`None`). At or above it,
/// only the users a search can return are loaded: a user session's own entry, the user of a user
/// DN `search_base`, or the users of `uid` / `mail` equality filters, at most [MAX_TARGETS]. Any
/// other search is refused with `sizeLimitExceeded`.
fn targets(
    user_count: i64,
    threshold: u16,
    user_id: Option<&str>,
    search_base: &str,
    filter: &LdapFilter,
) -> Result<Option<Vec<Target>>, (LdapResultCode, String)> {
    if user_count < threshold as i64 {
        return Ok(None);
    }
    if let Some(id) = user_id {
        return Ok(Some(vec![Target::Id(id.to_string())]));
    }
    let size_limit = || {
        (
            LdapResultCode::SizeLimitExceeded,
            "too many users, filter by uid or mail".to_string(),
        )
    };
    let base_uid = dn::parse(search_base).and_then(|rdns| match rdns.as_slice() {
        [(k, uid), (ou, people), ..] if k == "uid" && ou == "ou" && people == "people" => {
            Some(vec![Target::Uid(uid.clone())])
        }
        _ => None,
    });
    let mut targets = base_uid
        .or_else(|| filter_targets(filter))
        .ok_or_else(size_limit)?;
    targets.sort();
    targets.dedup();
    if targets.len() > MAX_TARGETS {
        return Err(size_limit());
    }
    Ok(Some(targets))
}

/// Returns the users that every entry matching `filter` must be one of, if the filter limits it.
fn filter_targets(filter: &LdapFilter) -> Option<Vec<Target>> {
    match filter {
        LdapFilter::Equality(attr, val) if attr.eq_ignore_ascii_case("uid") => {
            Some(vec![Target::Uid(val.to_lowercase())])
        }
        LdapFilter::Equality(attr, val) if attr.eq_ignore_ascii_case("mail") => {
            Some(vec![Target::Mail(val.to_lowercase())])
        }
        LdapFilter::And(fs) => fs.iter().find_map(filter_targets),
        LdapFilter::Or(fs) if !fs.is_empty() => {
            let mut targets = Vec::new();
            for f in fs {
                targets.extend(filter_targets(f)?);
            }
            Some(targets)
        }
        _ => None,
    }
}

/// Normalizes the requested search `base` and refuses any base outside `client_base`.
fn search_base(base: &str, client_base: &str) -> Result<String, (LdapResultCode, String)> {
    let client_base_norm = dn::normalize(client_base)
        .ok_or((LdapResultCode::Other, "invalid client base DN".into()))?;
    let search_base =
        dn::normalize(base).ok_or((LdapResultCode::InvalidDNSyntax, "invalid base DN".into()))?;
    if !dn::is_within(&search_base, &client_base_norm) {
        return Err((
            LdapResultCode::InsufficentAccessRights,
            format!("searches are limited to {client_base}"),
        ));
    }
    Ok(search_base)
}

/// Keeps the entries in `scope` of the normalized `search_base` that match `filter`. A `user_id`
/// limits them to that user's own entry.
fn visible(
    entries: Vec<Entry>,
    search_base: &str,
    scope: LdapSearchScope,
    user_id: Option<&str>,
    filter: &LdapFilter,
) -> Result<Vec<Entry>, (LdapResultCode, String)> {
    // user sessions get no `noSuchObject`, it would tell them which other entries exist
    if user_id.is_none()
        && !entries
            .iter()
            .any(|e| dn::normalize(&e.dn).as_deref() == Some(search_base))
    {
        return Err((LdapResultCode::NoSuchObject, String::new()));
    }

    let base_depth = dn::depth(search_base);
    Ok(entries
        .into_iter()
        .filter(|e| {
            let Some(norm) = dn::normalize(&e.dn) else {
                return false;
            };
            let in_scope = match scope {
                LdapSearchScope::Base => norm == search_base,
                LdapSearchScope::OneLevel => {
                    dn::is_within(&norm, search_base) && dn::depth(&norm) == base_depth + 1
                }
                LdapSearchScope::Subtree => dn::is_within(&norm, search_base),
                LdapSearchScope::Children => {
                    dn::is_within(&norm, search_base) && norm != search_base
                }
            };
            in_scope
                && user_id.is_none_or(|id| e.values("entryUUID").any(|v| v == id))
                && filter::matches(filter, e)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directory::DirUser;

    const BASE: &str = "cn=app,ou=clients,dc=x";

    fn entries() -> Vec<Entry> {
        let user = |id: &str, uid: &str| DirUser {
            id: id.into(),
            uid: uid.into(),
            email: format!("{uid}@x.io"),
            given_name: uid.into(),
            family_name: None,
            groups: vec!["admins".into()],
            posix: None,
        };
        let users = [user("id1", "alice"), user("id2", "bob")];
        directory::build("app", BASE, &users, &["admins".into()])
    }

    fn dns(
        base: &str,
        scope: LdapSearchScope,
        user_id: Option<&str>,
    ) -> Result<Vec<String>, LdapResultCode> {
        let search_base = search_base(base, BASE).map_err(|(c, _)| c)?;
        let all = LdapFilter::Present("objectClass".into());
        visible(entries(), &search_base, scope, user_id, &all)
            .map(|es| es.into_iter().map(|e| e.dn).collect())
            .map_err(|(c, _)| c)
    }

    #[test]
    fn search_boundary() {
        let alice = "uid=alice,ou=people,cn=app,ou=clients,dc=x";
        let bob = "uid=bob,ou=people,cn=app,ou=clients,dc=x";

        // service sessions see the whole subtree
        assert_eq!(dns(BASE, LdapSearchScope::Subtree, None).unwrap().len(), 6);
        assert_eq!(
            dns(BASE, LdapSearchScope::Base, None).unwrap(),
            vec![BASE.to_string()]
        );
        assert_eq!(
            dns(
                "ou=people,cn=app,ou=clients,dc=x",
                LdapSearchScope::OneLevel,
                None
            )
            .unwrap(),
            vec![alice, bob]
        );
        assert_eq!(dns(BASE, LdapSearchScope::OneLevel, None).unwrap().len(), 2);
        assert_eq!(dns(BASE, LdapSearchScope::Children, None).unwrap().len(), 5);
        assert_eq!(
            dns(
                "uid=nobody,ou=people,cn=app,ou=clients,dc=x",
                LdapSearchScope::Base,
                None
            ),
            Err(LdapResultCode::NoSuchObject)
        );

        // user sessions only ever see their own entry
        assert_eq!(
            dns(BASE, LdapSearchScope::Subtree, Some("id1")).unwrap(),
            vec![alice]
        );
        assert!(
            dns(bob, LdapSearchScope::Base, Some("id1"))
                .unwrap()
                .is_empty()
        );
        assert!(
            dns(
                "uid=nobody,ou=people,cn=app,ou=clients,dc=x",
                LdapSearchScope::Base,
                Some("id1")
            )
            .unwrap()
            .is_empty()
        );

        // nothing outside the client's subtree
        for base in ["ou=clients,dc=x", "cn=other,ou=clients,dc=x", "dc=x"] {
            assert_eq!(
                dns(base, LdapSearchScope::Subtree, None),
                Err(LdapResultCode::InsufficentAccessRights)
            );
        }
        assert_eq!(
            dns("not a dn", LdapSearchScope::Subtree, None),
            Err(LdapResultCode::InvalidDNSyntax)
        );
    }

    #[test]
    fn ssp_threshold_targets() {
        let eq = |a: &str, v: &str| LdapFilter::Equality(a.into(), v.into());
        let all = LdapFilter::Present("objectClass".into());

        // below the threshold, everything is loaded
        assert_eq!(targets(999, 1000, None, BASE, &all), Ok(None));

        // at or above it, user sessions only load their own entry
        assert_eq!(
            targets(1000, 1000, Some("id1"), BASE, &all),
            Ok(Some(vec![Target::Id("id1".into())]))
        );

        // service sessions need a uid / mail equality filter
        let filter = LdapFilter::And(vec![
            LdapFilter::Equality("objectClass".into(), "inetOrgPerson".into()),
            LdapFilter::Or(vec![eq("UID", "alice"), eq("mail", "alice@x.io")]),
        ]);
        assert_eq!(
            targets(1000, 1000, None, BASE, &filter),
            Ok(Some(vec![
                Target::Uid("alice".into()),
                Target::Mail("alice@x.io".into())
            ]))
        );
        for filter in [
            all.clone(),
            eq("cn", "alice"),
            LdapFilter::Or(vec![eq("uid", "alice"), eq("cn", "bob")]),
            LdapFilter::Not(Box::new(eq("uid", "alice"))),
            LdapFilter::Or(vec![]),
            LdapFilter::Or(
                (0..=MAX_TARGETS)
                    .map(|i| eq("uid", &i.to_string()))
                    .collect(),
            ),
        ] {
            assert_eq!(
                targets(1000, 1000, None, BASE, &filter).map_err(|(c, _)| c),
                Err(LdapResultCode::SizeLimitExceeded)
            );
        }

        // exactly MAX_TARGETS distinct targets are still accepted
        let filter = LdapFilter::Or(
            (0..MAX_TARGETS)
                .map(|i| eq("uid", &i.to_string()))
                .collect(),
        );
        assert_eq!(
            targets(1000, 1000, None, BASE, &filter).map(|t| t.map(|t| t.len())),
            Ok(Some(MAX_TARGETS))
        );

        // identical terms count once, case-insensitively like they are resolved
        let filter = LdapFilter::Or(
            (0..10_000)
                .map(|i| eq("uid", if i % 2 == 0 { "Alice" } else { "alice" }))
                .collect(),
        );
        assert_eq!(
            targets(1000, 1000, None, BASE, &filter),
            Ok(Some(vec![Target::Uid("alice".into())]))
        );

        // a base search on a user DN targets that user, whatever the filter
        let alice = "uid=alice,ou=people,cn=app,ou=clients,dc=x";
        assert_eq!(
            targets(1000, 1000, None, alice, &all),
            Ok(Some(vec![Target::Uid("alice".into())]))
        );
    }
}
