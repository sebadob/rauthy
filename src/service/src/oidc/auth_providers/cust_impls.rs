use rauthy_common::constants::APPLICATION_JSON;
use rauthy_common::http_client;
use rauthy_data::entity::auth_providers::AuthProviderIdClaims;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize)]
struct GitHubEmailPrivateResponse {
    email: String,
    primary: bool,
    verified: bool,
    // The visibility exists as well, but we don't care about it at all - no need to deserialize
    // visibility: Option<String>,
}

/// GitHub is very special and does its own thing, which is super annoying.
/// If a user has no public E-Mail and changed the visibility settings, the
/// user info endpoint will not return any address, even if a valid access token
/// has been provided. In this case, there is another special endpoint only for
/// E-Mail addresses which needs to be used to actually retrieve the address.
/// This means we need a 3rd request to GitHub.
///
/// Note: The user endpoint is hardcoded because it is very unlikely to ever
/// change in the future. If we allowed this to be customizable, everything
/// would get super messy. If the GitHub API ever updates, we just need to update
/// the URL here as well.
pub async fn get_github_private_email(
    access_token: &str,
    claims: &mut AuthProviderIdClaims<'_>,
) -> Result<(), ErrorResponse> {
    debug!("Trying to get User E-Mail via GitHub /user/emails endpoint");

    let res = http_client()
        .get("https://api.github.com/user/emails")
        .header(AUTHORIZATION, format!("Bearer {access_token}"))
        .header(ACCEPT, APPLICATION_JSON)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;

    let status = res.status().as_u16();
    debug!("GET /user/emails status: {status}\n{res:?}");

    if status < 300 {
        let mut emails = res.json::<Vec<GitHubEmailPrivateResponse>>().await?;
        debug!("GET /user/emails status: {status}");

        if emails.len() == 1 {
            let email = emails.swap_remove(0);
            claims.email = Some(email.email.into());
            claims.email_verified = Some(email.verified);
            return Ok(());
        }

        for email in emails {
            if email.primary {
                claims.email = Some(email.email.into());
                claims.email_verified = Some(email.verified);
                return Ok(());
            }
        }

        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "Could not find a the primary user E-Mail in GitHub response",
        ))
    } else {
        let text = res.text().await?;
        Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            format!("Error during User E-Mails lookup: {text}"),
        ))
    }
}
