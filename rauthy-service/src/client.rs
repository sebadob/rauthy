use actix_web::web;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::request::UpdateClientRequest;
use rauthy_models::response::ClientSecretResponse;

/// Updates a client.<br>
/// A client secret will be automatically generated if the
/// [UpdateClientRequest](crate::models::request::UpdateClientRequest) is set to be confidential
/// while the currently existing client does not have it. It will be skipped, if it was
/// `confidential` already.
pub async fn update_client(
    data: &web::Data<AppState>,
    id: String,
    client_req: UpdateClientRequest,
) -> Result<Client, ErrorResponse> {
    let mut client = Client::find(data, id).await?;
    if client.id != client_req.id {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("The 'id' cannot be changed"),
        ));
    }

    // do not allow too long auth code lifetime
    if client_req.auth_code_lifetime > 300 {}

    client.name = client_req.name;
    if client_req.confidential {
        // only set a new secret if this value has been changed
        if !client.confidential {
            let (_, enc) = Client::generate_new_secret(data)?;
            client.secret = Some(enc);
        }
    } else {
        client.secret = None;
    }
    client.confidential = client_req.confidential;

    client.redirect_uris = client_req.redirect_uris.join(",");
    client.post_logout_redirect_uris = client_req.post_logout_redirect_uris.map(|u| u.join(","));
    if let Some(origins) = client_req.allowed_origins {
        client.allowed_origins = Some(origins.join(","));
    }

    client.enabled = client_req.enabled;
    client.flows_enabled = client_req.flows_enabled.join(",");

    client.access_token_alg = client_req.access_token_alg;
    client.id_token_alg = client_req.id_token_alg;
    client.refresh_token = client_req.refresh_token;

    client.auth_code_lifetime = client_req.auth_code_lifetime;
    client.access_token_lifetime = client_req.access_token_lifetime;

    client.scopes = Client::sanitize_scopes(data, client_req.scopes).await?;
    client.default_scopes = Client::sanitize_scopes(data, client_req.default_scopes).await?;

    client.challenge = client_req.challenges.map(|c| c.join(","));

    client.save(data, None).await?;
    Ok(client)
}

/// Returns the clients secret in cleartext.
pub async fn get_client_secret(
    id: String,
    data: &web::Data<AppState>,
) -> Result<ClientSecretResponse, ErrorResponse> {
    let client = Client::find(data, id).await?;

    if !client.confidential {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            format!("'{}' is a public client", &client.id),
        ));
    }
    let secret = client.get_secret_cleartext(data)?;

    Ok(ClientSecretResponse {
        id: client.id,
        confidential: client.confidential,
        secret,
    })
}

/// Generates a new client secret and returns it then as clear text wrapped in a
/// [ClientSecretResponse](crate::models::response::ClientSecretResponse)
pub async fn generate_new_secret(
    id: String,
    data: &web::Data<AppState>,
) -> Result<ClientSecretResponse, ErrorResponse> {
    let mut client = Client::find(data, id).await?;
    let (clear, enc) = Client::generate_new_secret(data)?;

    client.confidential = true;
    client.secret = Some(enc);
    client.save(data, None).await?;

    Ok(ClientSecretResponse {
        id: client.id,
        confidential: true,
        secret: Some(clear),
    })
}
