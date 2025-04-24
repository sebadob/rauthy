use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::auth_providers::AuthProvider;
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::config::ConfigEntity;
use crate::entity::devices::DeviceEntity;
use crate::entity::failed_backchannel_logout::FailedBackchannelLogout;
use crate::entity::groups::Group;
use crate::entity::jwk::Jwk;
use crate::entity::logos::Logo;
use crate::entity::magic_links::MagicLink;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pictures::UserPicture;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use crate::entity::roles::Role;
use crate::entity::scopes::Scope;
use crate::entity::sessions::Session;
use crate::entity::theme::ThemeCssFull;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::user_login_states::UserLoginState;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::events::event::Event;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;

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
mfa_claim_value, allow_insecure_requests, use_pkce, root_pem, client_secret_basic, client_secret_post,
jwks_endpoint)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)"#;

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
                        b.allow_insecure_requests,
                        b.use_pkce,
                        b.root_pem,
                        b.client_secret_basic,
                        b.client_secret_post,
                        b.jwks_endpoint
                    ),
                )
                .await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(
                sql_1,
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
                    &b.allow_insecure_requests,
                    &b.use_pkce,
                    &b.root_pem,
                    &b.client_secret_basic,
                    &b.client_secret_post,
                    &b.jwks_endpoint,
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
backchannel_logout_uri)
VALUES
($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)"#;

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
                        b.backchannel_logout_uri
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
                sql_1,
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

pub async fn password_policy(bytes: Vec<u8>) -> Result<(), ErrorResponse> {
    let sql = "UPDATE config SET data = $1 WHERE id = 'password_policy'";

    if is_hiqlite() {
        DB::hql().execute(sql, params!(bytes)).await?;
    } else {
        DB::pg_execute(sql, &[]).await?;
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
                    &b.version,
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

pub async fn user_attr_config(data_before: Vec<UserAttrConfigEntity>) -> Result<(), ErrorResponse> {
    let sql_1 = "DELETE FROM user_attr_config";
    let sql_2 = "INSERT INTO user_attr_config (name, desc) VALUES ($1, $2)";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        for b in data_before {
            DB::hql().execute(sql_2, params!(b.name, b.desc)).await?;
        }
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        for b in data_before {
            DB::pg_execute(sql_2, &[&b.name, &b.desc]).await?;
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
users_values (id, birthdate, phone, street, zip, city, country)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

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
                        b.country
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
                ],
            )
            .await?;
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
