use rauthy_api_types::clients::{ClientSecretResponse, UpdateClientRequest};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_scim::ClientScim;

/// Returns `true` inside `Option<(ClientScim, bool)>` if `ClientScim`
/// has been updated and therefore needs a full sync.
pub async fn update_client(
    id: String,
    client_req: UpdateClientRequest,
) -> Result<(Client, Option<(ClientScim, bool)>), ErrorResponse> {
    let mut client = Client::find(id).await?;
    if client.id != client_req.id {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The 'id' cannot be changed",
        ));
    }

    client.name = client_req.name;
    if client_req.confidential {
        // only set a new secret if this value has been changed
        if !client.confidential {
            let (_, enc) = Client::generate_new_secret()?;
            client.secret = Some(enc);
        }
    } else {
        client.secret = None;
    }
    client.confidential = client_req.confidential;

    client.redirect_uris = client_req.redirect_uris.join(",");
    client.post_logout_redirect_uris = client_req.post_logout_redirect_uris.map(|u| u.join(","));
    client.allowed_origins = client_req.allowed_origins.map(|o| o.join(","));

    client.enabled = client_req.enabled;
    client.flows_enabled = client_req.flows_enabled.join(",");

    client.access_token_alg = client_req.access_token_alg.to_string();
    client.id_token_alg = client_req.id_token_alg.to_string();

    client.auth_code_lifetime = client_req.auth_code_lifetime;
    client.access_token_lifetime = client_req.access_token_lifetime;

    let scopes_before = client.scopes;
    client.scopes = Client::sanitize_scopes(client_req.scopes).await?;
    let default_scopes_before = client.default_scopes;
    client.default_scopes = Client::sanitize_scopes(client_req.default_scopes).await?;
    let scopes_diff =
        scopes_before != client.scopes || default_scopes_before != client.default_scopes;

    client.challenge = client_req.challenges.map(|c| c.join(","));
    client.force_mfa = client_req.force_mfa;

    client.contacts = client_req.contacts.map(|c| c.join(","));
    client.client_uri = client_req.client_uri;
    client.backchannel_logout_uri = client_req.backchannel_logout_uri;
    client.restrict_group_prefix = client_req.restrict_group_prefix;
    client.cust_email_mapping = client_req.cust_email_mapping;

    client.save().await?;

    let scim = if let Some(scim_req) = client_req.scim {
        let base_uri = scim_req
            .base_uri
            .strip_suffix("/")
            .map(String::from)
            .unwrap_or(scim_req.base_uri);

        if let Some(scim_before) = ClientScim::find_opt(client.id.clone()).await? {
            let scim_new = ClientScim {
                client_id: scim_before.client_id.clone(),
                bearer_token: scim_req.bearer_token.clone(),
                base_uri,
                sync_groups: scim_req.sync_groups,
                group_sync_prefix: scim_req.group_sync_prefix.clone(),
            };
            let needs_sync = scim_new != scim_before;

            if needs_sync {
                ClientScim::upsert(
                    scim_before.client_id,
                    scim_req.bearer_token.as_bytes(),
                    scim_new.base_uri.clone(),
                    scim_req.sync_groups,
                    scim_req.group_sync_prefix.clone(),
                )
                .await?;
            }

            Some((scim_new, needs_sync || scopes_diff))
        } else {
            ClientScim::upsert(
                client.id.clone(),
                scim_req.bearer_token.as_bytes(),
                base_uri.clone(),
                scim_req.sync_groups,
                scim_req.group_sync_prefix.clone(),
            )
            .await?;

            let scim_new = ClientScim {
                client_id: client.id.clone(),
                bearer_token: scim_req.bearer_token,
                base_uri,
                sync_groups: scim_req.sync_groups,
                group_sync_prefix: scim_req.group_sync_prefix,
            };

            Some((scim_new, true))
        }
    } else {
        ClientScim::delete(client.id.clone()).await?;
        None
    };

    Ok((client, scim))
}

/// Returns the clients secret in cleartext.
pub async fn get_client_secret(id: String) -> Result<ClientSecretResponse, ErrorResponse> {
    let client = Client::find(id).await?;

    if !client.confidential {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            format!("'{}' is a public client", &client.id),
        ));
    }
    let secret = client.get_secret_cleartext()?;

    Ok(ClientSecretResponse {
        id: client.id,
        confidential: client.confidential,
        secret,
    })
}

pub async fn generate_new_secret(
    id: String,
    cache_current_hours: Option<u8>,
) -> Result<ClientSecretResponse, ErrorResponse> {
    let mut client = Client::find(id).await?;

    client.cache_current_secret(cache_current_hours).await?;

    let (clear, enc) = Client::generate_new_secret()?;
    client.confidential = true;
    client.secret = Some(enc);
    client.save().await?;

    Ok(ClientSecretResponse {
        id: client.id,
        confidential: true,
        secret: Some(clear),
    })
}
