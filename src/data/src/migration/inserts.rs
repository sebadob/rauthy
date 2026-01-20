use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::auth_providers::AuthProvider;
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::clients_scim::ClientScim;
use crate::entity::config::ConfigEntity;
use crate::entity::devices::DeviceEntity;
use crate::entity::email_jobs::EmailJob;
use crate::entity::failed_backchannel_logout::FailedBackchannelLogout;
use crate::entity::failed_scim_tasks::FailedScimTask;
use crate::entity::groups::Group;
use crate::entity::issued_tokens::IssuedToken;
use crate::entity::jwk::Jwk;
use crate::entity::login_locations::LoginLocation;
use crate::entity::logos::Logo;
use crate::entity::magic_links::MagicLink;
use crate::entity::pam::authorized_keys::AuthorizedKey;
use crate::entity::pam::groups::PamGroup;
use crate::entity::pam::hosts::PamHost;
use crate::entity::pam::users::PamUser;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pictures::UserPicture;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use crate::entity::roles::Role;
use crate::entity::scopes::Scope;
use crate::entity::sessions::Session;
use crate::entity::theme::ThemeCssFull;
use crate::entity::tos::ToS;
use crate::entity::tos_user_accept::ToSUserAccept;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::user_login_states::UserLoginState;
use crate::entity::user_revoke::UserRevoke;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::events::event::Event;
use cryptr::EncValue;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use std::cmp::max;

pub async fn api_keys(data_before: Vec<ApiKeyEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM api_keys";
    let sql_2 = r#"
INSERT INTO
api_keys (name, secret, created, expires, enc_key_id, access)
VALUES ($1, $2, $3, $4, $5, $6)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.name,
                        b.secret,
                        b.created,
                        b.expires,
                        b.enc_key_id,
                        b.access
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.name,
                    &b.secret,
                    &b.created,
                    &b.expires,
                    &b.enc_key_id,
                    &b.access,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn auth_provider_logos(data_before: Vec<Logo>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM auth_provider_logos";
    let sql_2 = r#"
INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data, updated)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT(auth_provider_id, res) DO UPDATE
SET content_type = $3, data = $4, updated = $5"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.id, b.res.as_str(), b.content_type, b.data, b.updated),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.id, &b.res.as_str(), &b.content_type, &b.data, &b.updated],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn auth_providers(data_before: Vec<AuthProvider>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM auth_providers";
    let sql_2 = r#"
INSERT INTO
auth_providers (id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value, mfa_claim_path,
mfa_claim_value, use_pkce, client_secret_basic, client_secret_post, jwks_endpoint, auto_onboarding,
auto_link)
VALUES (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21
)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.enabled,
                        b.name,
                        b.typ.as_str(),
                        b.issuer,
                        b.authorization_endpoint,
                        b.token_endpoint,
                        b.userinfo_endpoint,
                        b.client_id,
                        b.secret,
                        b.scope,
                        b.admin_claim_path,
                        b.admin_claim_value,
                        b.mfa_claim_path,
                        b.mfa_claim_value,
                        b.use_pkce,
                        b.client_secret_basic,
                        b.client_secret_post,
                        b.jwks_endpoint,
                        b.auto_onboarding,
                        b.auto_link
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.enabled,
                    &b.name,
                    &b.typ.as_str(),
                    &b.issuer,
                    &b.authorization_endpoint,
                    &b.token_endpoint,
                    &b.userinfo_endpoint,
                    &b.client_id,
                    &b.secret,
                    &b.scope,
                    &b.admin_claim_path,
                    &b.admin_claim_value,
                    &b.mfa_claim_path,
                    &b.mfa_claim_value,
                    &b.use_pkce,
                    &b.client_secret_basic,
                    &b.client_secret_post,
                    &b.jwks_endpoint,
                    &b.auto_onboarding,
                    &b.auto_link,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn client_logos(data_before: Vec<Logo>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM client_logos";
    let sql_2 = r#"
INSERT INTO client_logos (client_id, res, content_type, data, updated)
VALUES ($1, $2, $3, $4, $5)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.id, b.res.as_str(), b.content_type, b.data, b.updated),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.id, &b.res.as_str(), &b.content_type, &b.data, &b.updated],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn clients(data_before: Vec<Client>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM clients";
    let sql_2 = r#"
INSERT INTO clients
(id, name, enabled, confidential, secret, secret_kid, redirect_uris, post_logout_redirect_uris,
allowed_origins, flows_enabled, access_token_alg, id_token_alg, auth_code_lifetime,
access_token_lifetime, scopes, default_scopes, challenge, force_mfa, client_uri, contacts,
backchannel_logout_uri, restrict_group_prefix)
VALUES
($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;

        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.name,
                        b.enabled,
                        b.confidential,
                        b.secret,
                        b.secret_kid,
                        b.redirect_uris,
                        b.post_logout_redirect_uris,
                        b.allowed_origins,
                        b.flows_enabled,
                        b.access_token_alg,
                        b.id_token_alg,
                        b.auth_code_lifetime,
                        b.access_token_lifetime,
                        b.scopes,
                        b.default_scopes,
                        b.challenge,
                        b.force_mfa,
                        b.client_uri,
                        b.contacts,
                        b.backchannel_logout_uri,
                        b.restrict_group_prefix
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.name,
                    &b.enabled,
                    &b.confidential,
                    &b.secret,
                    &b.secret_kid,
                    &b.redirect_uris,
                    &b.post_logout_redirect_uris,
                    &b.allowed_origins,
                    &b.flows_enabled,
                    &b.access_token_alg,
                    &b.id_token_alg,
                    &b.auth_code_lifetime,
                    &b.access_token_lifetime,
                    &b.scopes,
                    &b.default_scopes,
                    &b.challenge,
                    &b.force_mfa,
                    &b.client_uri,
                    &b.contacts,
                    &b.backchannel_logout_uri,
                    &b.restrict_group_prefix,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn clients_dyn(data_before: Vec<ClientDyn>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM clients_dyn";
    let sql_2 = r#"
INSERT INTO
clients_dyn (id, created, last_used, registration_token, token_endpoint_auth_method)
VALUES ($1, $2, $3, $4, $5)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.created,
                        b.last_used,
                        b.registration_token,
                        b.token_endpoint_auth_method
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.created,
                    &b.last_used,
                    &b.registration_token,
                    &b.token_endpoint_auth_method,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn clients_scim(data_before: Vec<ClientScim>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM clients_scim";
    let sql_2 = r#"
INSERT INTO clients_scim (client_id, bearer_token, base_endpoint, sync_groups, group_sync_prefix)
VALUES ($1, $2, $3, $4, $5)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            let bearer_encrypted = EncValue::encrypt(b.bearer_token.as_bytes())?.into_bytes();
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.client_id,
                        bearer_encrypted.as_ref(),
                        b.base_uri,
                        b.sync_groups,
                        b.group_sync_prefix
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            let bearer_encrypted = EncValue::encrypt(b.bearer_token.as_bytes())?.into_bytes();
            DB::pg_execute(
                sql_2,
                &[
                    &b.client_id,
                    &bearer_encrypted.as_ref(),
                    &b.base_uri,
                    &b.sync_groups,
                    &b.group_sync_prefix,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn config(data_before: Vec<ConfigEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM config";
    let sql_2 = "INSERT INTO config (id, data) VALUES ($1, $2)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql().execute(sql_2, params!(b.id, b.data)).await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.id, &b.data]).await?;
        }
    }
    Ok(())
}

pub async fn devices(data_before: Vec<DeviceEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM devices";
    let sql_2 = r#"
INSERT INTO devices
(id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.client_id,
                        b.user_id,
                        b.created,
                        b.access_exp,
                        b.refresh_exp,
                        b.peer_ip,
                        b.name
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.client_id,
                    &b.user_id,
                    &b.created,
                    &b.access_exp,
                    &b.refresh_exp,
                    &b.peer_ip,
                    &b.name,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn email_jobs(data_before: Vec<EmailJob>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM email_jobs";
    let sql_2 = r#"
INSERT INTO email_jobs (id, scheduled, status, updated, filter, content_type, subject, body)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.scheduled,
                        b.status.value(),
                        b.updated,
                        b.filter.to_string(),
                        b.content_type.to_string(),
                        b.subject,
                        b.body
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.scheduled,
                    &b.status.value(),
                    &b.updated,
                    &b.filter.to_string(),
                    &b.content_type.to_string(),
                    &b.subject,
                    &b.body,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn events(data_before: Vec<Event>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM events";
    let sql_2 = r#"
INSERT INTO events (id, timestamp, level, typ, ip, data, text)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.timestamp,
                        b.level.value(),
                        b.typ.value(),
                        b.ip,
                        b.data,
                        b.text
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.timestamp,
                    &b.level.value(),
                    &b.typ.value(),
                    &b.ip,
                    &b.data,
                    &b.text,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn failed_backchannel_logouts(
    data_before: Vec<FailedBackchannelLogout>,
) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM failed_backchannel_logouts";
    let sql_2 = r#"
INSERT INTO failed_backchannel_logouts (client_id, sub, sid, retry_count)
VALUES ($1, $2, $3, $4)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;

        for b in data_before {
            DB::hql()
                .execute(sql_2, params!(b.client_id, b.sub, b.sid, b.retry_count))
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.client_id, &b.sub, &b.sid, &b.retry_count]).await?;
        }
    }
    Ok(())
}

pub async fn failed_scim_tasks(data_before: Vec<FailedScimTask>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM failed_scim_tasks";
    let sql_2 = r#"
INSERT INTO failed_scim_tasks (client_id, action, retry_count)
VALUES ($1, $2, $3)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;

        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.client_id, b.action.to_string(), b.retry_count),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.client_id, &b.action.to_string(), &(b.retry_count as i32)],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn groups(data_before: Vec<Group>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM groups";
    let sql_2 = "INSERT INTO groups (id, name) VALUES ($1, $2)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql().execute(sql_2, params!(b.id, b.name)).await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.id, &b.name]).await?;
        }
    }
    Ok(())
}

pub async fn issued_tokens(data_before: Vec<IssuedToken>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM issued_tokens";
    let sql_2 = r#"
INSERT INTO issued_tokens (jti, user_id, did, sid, exp, revoked)
VALUES ($1, $2, $3, $4, $5, $6)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.jti, b.user_id, b.did, b.sid, b.exp, b.revoked),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.jti, &b.user_id, &b.did, &b.sid, &b.exp, &b.revoked],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn jwks(data_before: Vec<Jwk>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM jwks";
    let sql_2 = r#"
INSERT INTO jwks (kid, created_at, signature, enc_key_id, jwk)
VALUES ($1, $2, $3, $4, $5)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.kid,
                        b.created_at,
                        b.signature.as_str(),
                        &b.enc_key_id,
                        b.jwk
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.kid,
                    &b.created_at,
                    &b.signature.as_str(),
                    &b.enc_key_id,
                    &b.jwk,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn login_locations(data_before: Vec<LoginLocation>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM login_locations";
    let sql_2 = r#"
INSERT INTO login_locations (user_id, browser_id, ip, last_seen, user_agent, location)
VALUES ($1, $2, $3, $4, $5, $6)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;

        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.user_id,
                        b.browser_id,
                        b.ip,
                        b.last_seen,
                        b.user_agent,
                        b.location
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.user_id,
                    &b.browser_id,
                    &b.ip,
                    &b.last_seen,
                    &b.user_agent,
                    &b.location,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn magic_links(data_before: Vec<MagicLink>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM magic_links";
    let sql_2 = r#"
INSERT INTO magic_links
(id, user_id, csrf_token, cookie, exp, used, usage)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.user_id,
                        b.csrf_token,
                        b.cookie,
                        b.exp,
                        b.used,
                        b.usage
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.user_id,
                    &b.csrf_token,
                    &b.cookie,
                    &b.exp,
                    &b.used,
                    &b.usage,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn pam_groups(data_before: Vec<PamGroup>) -> Result<(), ErrorResponse> {
    // To resolve FK issues, we will delete all pam tables here, since this will be the
    // first inserts that will be done.

    let sql_1 = "DELETE FROM pam_rel_groups_users";
    let sql_2 = "DELETE FROM pam_users";
    let sql_3 = "DELETE FROM pam_hosts_ips";
    let sql_4 = "DELETE FROM pam_hosts_aliases";
    let sql_5 = "DELETE FROM pam_hosts";
    let sql_6 = "DELETE FROM pam_groups";

    let sql_7 = r#"
INSERT INTO pam_groups (id, name, typ)
VALUES ($1, $2, $3)"#;

    let mut highest_id = 0;
    for group in &data_before {
        highest_id = max(highest_id, group.id);
    }

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        DB::hql().execute(sql_2, params!()).await?;
        DB::hql().execute(sql_3, params!()).await?;
        DB::hql().execute(sql_4, params!()).await?;
        DB::hql().execute(sql_5, params!()).await?;
        DB::hql().execute(sql_6, params!()).await?;

        DB::hql()
            .execute(
                "DELETE FROM sqlite_sequence WHERE name = $1",
                params!("pam_groups"),
            )
            .await?;

        for b in data_before {
            DB::hql()
                .execute(sql_7, params!(b.id, b.name, b.typ.as_str()))
                .await?;
        }

        highest_id = max(highest_id, 99999);
        // in any other case, SQLite tracks the correct id on manual insert anyway
        if highest_id == 99999 {
            DB::hql()
                .execute(
                    r#"
        INSERT INTO sqlite_sequence (name, seq)
        VALUES ($1, $2)
        "#,
                    params!("pam_groups", highest_id),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        DB::pg_execute(sql_2, &[]).await?;
        DB::pg_execute(sql_3, &[]).await?;
        DB::pg_execute(sql_4, &[]).await?;
        DB::pg_execute(sql_5, &[]).await?;
        DB::pg_execute(sql_6, &[]).await?;

        for b in data_before {
            DB::pg_execute(sql_7, &[&(b.id as i64), &b.name, &b.typ.as_str()]).await?;
        }

        highest_id = max(highest_id + 1, 100000);
        let stmt = format!("ALTER SEQUENCE pam_groups_id_seq RESTART WITH {highest_id}");
        DB::pg_execute(&stmt, &[]).await?;
    }
    Ok(())
}

pub async fn pam_hosts(data_before: Vec<PamHost>) -> Result<(), ErrorResponse> {
    let sql_1 = r#"
INSERT INTO pam_hosts (id, hostname, gid, secret, force_mfa, local_password_only, notes)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

    if is_hiqlite() {
        for b in data_before {
            DB::hql()
                .execute(
                    sql_1,
                    params!(
                        b.id,
                        b.hostname,
                        b.gid,
                        b.secret,
                        b.force_mfa,
                        b.local_password_only,
                        b.notes
                    ),
                )
                .await?;
        }
    } else {
        for b in data_before {
            DB::pg_execute(
                sql_1,
                &[
                    &b.id,
                    &b.hostname,
                    &(b.gid as i64),
                    &b.secret,
                    &b.force_mfa,
                    &b.local_password_only,
                    &b.notes,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn pam_hosts_aliases(data_before: Vec<(String, String)>) -> Result<(), ErrorResponse> {
    let sql_1 = r#"
INSERT INTO pam_hosts_aliases (host_id, alias)
VALUES ($1, $2)"#;

    if is_hiqlite() {
        for b in data_before {
            DB::hql().execute(sql_1, params!(b.0, b.1)).await?;
        }
    } else {
        for b in data_before {
            DB::pg_execute(sql_1, &[&b.0, &b.1]).await?;
        }
    }
    Ok(())
}

pub async fn pam_hosts_ips(data_before: Vec<(String, String)>) -> Result<(), ErrorResponse> {
    let sql_1 = r#"
INSERT INTO pam_hosts_ips (host_id, ip)
VALUES ($1, $2)"#;

    if is_hiqlite() {
        for b in data_before {
            DB::hql().execute(sql_1, params!(b.0, b.1)).await?;
        }
    } else {
        for b in data_before {
            DB::pg_execute(sql_1, &[&b.0, &b.1]).await?;
        }
    }
    Ok(())
}

pub async fn pam_users(data_before: Vec<PamUser>) -> Result<(), ErrorResponse> {
    let sql_1 = r#"
INSERT INTO pam_users (id, name, gid, email, shell)
VALUES ($1, $2, $3, $4, $5)"#;

    let mut highest_id = 0;
    for user in &data_before {
        highest_id = max(highest_id, user.id);
    }

    if is_hiqlite() {
        DB::hql()
            .execute(
                "DELETE FROM sqlite_sequence WHERE name = $1",
                params!("pam_users"),
            )
            .await?;

        for b in data_before {
            DB::hql()
                .execute(sql_1, params!(b.id, b.name, b.gid, b.email, b.shell))
                .await?;
        }

        highest_id = max(highest_id, 99999);
        // in any other case, SQLite tracks the correct id on manual insert anyway
        if highest_id == 99999 {
            DB::hql()
                .execute(
                    r#"
    INSERT INTO sqlite_sequence (name, seq)
    VALUES ($1, $2)
    "#,
                    params!("pam_users", highest_id),
                )
                .await?;
        }
    } else {
        for b in data_before {
            DB::pg_execute(
                sql_1,
                &[&(b.id as i64), &b.name, &(b.gid as i64), &b.email, &b.shell],
            )
            .await?;
        }

        highest_id = max(highest_id + 1, 100000);
        let stmt = format!("ALTER SEQUENCE pam_users_id_seq RESTART WITH {highest_id}");
        DB::pg_execute(&stmt, &[]).await?;
    }
    Ok(())
}

pub async fn pam_rel_groups_users(data_before: Vec<(i64, i64, bool)>) -> Result<(), ErrorResponse> {
    let sql_1 = r#"
INSERT INTO pam_rel_groups_users (gid, uid, wheel)
VALUES ($1, $2, $3)"#;

    if is_hiqlite() {
        for b in data_before {
            DB::hql().execute(sql_1, params!(b.0, b.1, b.2)).await?;
        }
    } else {
        for b in data_before {
            DB::pg_execute(sql_1, &[&b.0, &b.1, &b.2]).await?;
        }
    }
    Ok(())
}

pub async fn passkeys(data_before: Vec<PasskeyEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM passkeys";
    let sql_2 = r#"
INSERT INTO passkeys
(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.user_id,
                        b.name,
                        b.passkey_user_id,
                        b.passkey,
                        b.credential_id,
                        b.registered,
                        b.last_used,
                        b.user_verified
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.user_id,
                    &b.name,
                    &b.passkey_user_id,
                    &b.passkey,
                    &b.credential_id,
                    &b.registered,
                    &b.last_used,
                    &b.user_verified,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn pictures(data_before: Vec<UserPicture>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM pictures";
    let sql_2 = r#"
INSERT INTO pictures (id, content_type, storage, data)
VALUES ($1, $2, $3, $4)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;

        for b in data_before {
            DB::hql()
                .execute(sql_2, params!(b.id, b.content_type, b.storage, b.data))
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.id, &b.content_type, &b.storage, &b.data]).await?;
        }
    }

    Ok(())
}

pub async fn recent_passwords(
    data_before: Vec<RecentPasswordsEntity>,
) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM recent_passwords";
    let sql_2 = "INSERT INTO recent_passwords (user_id, passwords) VALUES ($1, $2)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(sql_2, params!(b.user_id, b.passwords))
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.user_id, &b.passwords]).await?;
        }
    }
    Ok(())
}

pub async fn refresh_tokens(data_before: Vec<RefreshToken>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM refresh_tokens";
    let sql_2 = r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope, is_mfa, session_id)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.user_id,
                        b.nbf,
                        b.exp,
                        b.scope,
                        b.is_mfa,
                        b.session_id
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.user_id,
                    &b.nbf,
                    &b.exp,
                    &b.scope,
                    &b.is_mfa,
                    &b.session_id,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn refresh_tokens_devices(
    data_before: Vec<RefreshTokenDevice>,
) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM refresh_tokens_devices";
    let sql_2 = r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5, $6)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.id, b.device_id, b.user_id, b.nbf, b.exp, b.scope),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.id, &b.device_id, &b.user_id, &b.nbf, &b.exp, &b.scope],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn roles(data_before: Vec<Role>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM roles";
    let sql_2 = "INSERT INTO roles (id, name) VALUES ($1, $2)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql().execute(sql_2, params!(b.id, b.name)).await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.id, &b.name]).await?;
        }
    }
    Ok(())
}

pub async fn scopes(data_before: Vec<Scope>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM scopes";
    let sql_2 = r#"
INSERT INTO scopes (id, name, attr_include_access, attr_include_id)
VALUES ($1, $2, $3, $4)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.id, b.name, b.attr_include_access, b.attr_include_id),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.id, &b.name, &b.attr_include_access, &b.attr_include_id],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn sessions(data_before: Vec<Session>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM sessions";
    let sql_2 = r#"
INSERT INTO
sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.csrf_token,
                        b.user_id,
                        b.roles,
                        b.groups,
                        b.is_mfa,
                        b.state.as_str(),
                        b.exp,
                        b.last_seen
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.csrf_token,
                    &b.user_id,
                    &b.roles,
                    &b.groups,
                    &b.is_mfa,
                    &b.state.as_str(),
                    &b.exp,
                    &b.last_seen,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn ssh_auth_keys(data_before: Vec<AuthorizedKey>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM ssh_auth_keys";
    let sql_2 = r#"
INSERT INTO ssh_auth_keys (pam_uid, ts_added, expires, typ, data, comment)
VALUES ($1, $2, $3, $4, $5, $6)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.pam_uid, b.ts_added, b.expires, b.typ, b.data, b.comment),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &(b.pam_uid as i64),
                    &b.ts_added,
                    &b.expires,
                    &b.typ,
                    &b.data,
                    &b.comment,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn ssh_auth_keys_used(data_before: Vec<(String, i64)>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM ssh_auth_keys_used";
    let sql_2 = r#"
INSERT INTO ssh_auth_keys_used (used_key_hash, ts_added)
VALUES ($1, $2)
"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql().execute(sql_2, params!(b.0, b.1)).await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.0, &b.1]).await?;
        }
    }
    Ok(())
}

pub async fn themes(data_before: Vec<ThemeCssFull>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM themes";
    let sql_2 = r#"
INSERT INTO themes (client_id, last_update, version, light, dark, border_radius)
VALUES ($1, $2, $3, $4, $5, $6)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.client_id,
                        b.last_update,
                        b.version,
                        b.light.as_bytes(),
                        b.dark.as_bytes(),
                        b.border_radius
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.client_id,
                    &b.last_update,
                    &(b.version as i32),
                    &b.light.as_bytes(),
                    &b.dark.as_bytes(),
                    &b.border_radius,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn tos(data_before: Vec<ToS>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM tos";
    let sql_2 = r#"
INSERT INTO tos (ts, author, is_html, opt_until, content)
VALUES ($1, $2, $3, $4, $5)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.ts, b.author, b.is_html, b.opt_until, b.content),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.ts, &b.author, &b.is_html, &b.opt_until, &b.content],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn tos_user_accept(data_before: Vec<ToSUserAccept>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM tos_user_accept";
    let sql_2 = r#"
INSERT INTO tos_user_accept (user_id, tos_ts, accept_ts, location)
VALUES ($1, $2, $3, $4)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(sql_2, params!(b.user_id, b.tos_ts, b.accept_ts, b.location))
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.user_id, &b.tos_ts, &b.accept_ts, &b.location]).await?;
        }
    }
    Ok(())
}

pub async fn user_attr_config(data_before: Vec<UserAttrConfigEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM user_attr_config";
    let sql_2 = r#"
INSERT INTO user_attr_config (name, "desc", default_value, typ, user_editable)
VALUES ($1, $2, $3, $4, $5)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            let typ = b.typ.as_ref().map(|t| t.as_str());
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.name, b.desc, b.default_value, typ, b.user_editable),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            let typ = b.typ.as_ref().map(|t| t.as_str());
            DB::pg_execute(
                sql_2,
                &[&b.name, &b.desc, &b.default_value, &typ, &b.user_editable],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn user_attr_values(data_before: Vec<UserAttrValueEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM user_attr_values";
    let sql_2 = "INSERT INTO user_attr_values (user_id, key, value) VALUES ($1, $2, $3)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(sql_2, params!(b.user_id, b.key, b.value))
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.user_id, &b.key, &b.value]).await?;
        }
    }
    Ok(())
}

pub async fn user_login_states(data_before: Vec<UserLoginState>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM user_login_states";
    let sql_2 = r#"
INSERT INTO user_login_states(timestamp, user_id, client_id, session_id)
VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;

        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(b.timestamp, b.user_id, b.client_id, b.session_id),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[&b.timestamp, &b.user_id, &b.client_id, &b.session_id],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn users(data_before: Vec<User>) -> Result<(), ErrorResponse> {
    // the user_login_states restrict a deletion to prevent logic errors, which we can ignore
    // during a migration
    let sql_1 = "DELETE FROM user_login_states";
    let sql_2 = "DELETE FROM users";
    let sql_3 = r#"
INSERT INTO users
(id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
password_expires, created_at, last_login, last_failed_login, failed_login_attempts, language,
webauthn_user_id, user_expires, auth_provider_id, federation_uid, picture_id)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        DB::hql().execute(sql_2, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_3,
                    params!(
                        b.id,
                        b.email,
                        b.given_name,
                        b.family_name,
                        b.password,
                        b.roles,
                        b.groups,
                        b.enabled,
                        b.email_verified,
                        b.password_expires,
                        b.created_at,
                        b.last_login,
                        b.last_failed_login,
                        b.failed_login_attempts,
                        b.language.as_str(),
                        b.webauthn_user_id,
                        b.user_expires,
                        b.auth_provider_id,
                        b.federation_uid,
                        b.picture_id
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        DB::pg_execute(sql_2, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_3,
                &[
                    &b.id,
                    &b.email,
                    &b.given_name,
                    &b.family_name,
                    &b.password,
                    &b.roles,
                    &b.groups,
                    &b.enabled,
                    &b.email_verified,
                    &b.password_expires,
                    &b.created_at,
                    &b.last_login,
                    &b.last_failed_login,
                    &b.failed_login_attempts,
                    &b.language.as_str(),
                    &b.webauthn_user_id,
                    &b.user_expires,
                    &b.auth_provider_id,
                    &b.federation_uid,
                    &b.picture_id,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn users_values(data_before: Vec<UserValues>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM users_values";
    let sql_2 = r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country, preferred_username, tz)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(
                    sql_2,
                    params!(
                        b.id,
                        b.birthdate,
                        b.phone,
                        b.street,
                        b.zip,
                        b.city,
                        b.country,
                        b.preferred_username,
                        b.tz
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_2,
                &[
                    &b.id,
                    &b.birthdate,
                    &b.phone,
                    &b.street,
                    &b.zip,
                    &b.city,
                    &b.country,
                    &b.preferred_username,
                    &b.tz,
                ],
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn user_revoke(data_before: Vec<UserRevoke>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM user_revoke";
    let sql_2 = r#"
INSERT INTO user_revoke (user_id, code)
VALUES ($1, $2)"#;

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql().execute(sql_2, params!(b.user_id, b.code)).await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.user_id, &b.code]).await?;
        }
    }
    Ok(())
}

pub async fn webids(data_before: Vec<WebId>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM webids";
    let sql_2 = "INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql()
                .execute(sql_2, params!(b.user_id, b.custom_triples, b.expose_email))
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.user_id, &b.custom_triples, &b.expose_email]).await?;
        }
    }
    Ok(())
}
