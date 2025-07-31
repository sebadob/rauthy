use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use rauthy_common::HTTP_CLIENT;
use rauthy_common::constants::IDX_SMTP_OAUTH_TOKEN;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;
use tracing::{debug, error};

#[derive(Serialize)]
struct RequestPayload<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    scope: &'a str,
    grant_type: &'a str,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TokenResponse {
    token_type: String,
    expires_in: i64,
    ext_expires_in: i64,
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmtpOauthToken {
    pub access_token: String,
}

impl SmtpOauthToken {
    pub async fn get() -> Result<Self, ErrorResponse> {
        if !RauthyConfig::get().vars.email.auth_xoauth2 {
            panic!(
                "SmtpOauthToken fetch with no configured auth_xoauth2 - this should never happen"
            );
        }

        if let Some(slf) = DB::hql().get(Cache::App, IDX_SMTP_OAUTH_TOKEN).await? {
            return Ok(slf);
        }

        let token = Self::fetch().await?;
        let slf = Self {
            access_token: token.access_token,
        };

        if token.expires_in > 30 {
            DB::hql()
                .put(
                    Cache::App,
                    IDX_SMTP_OAUTH_TOKEN,
                    &slf,
                    Some(token.expires_in - 30),
                )
                .await?;
        }

        Ok(slf)
    }

    async fn fetch() -> Result<TokenResponse, ErrorResponse> {
        let vars = &RauthyConfig::get().vars.email;

        // all these are validated during RauthyConfig build at startup
        let url = vars.xoauth_url.as_ref().unwrap();
        let client_id = vars.xoauth_client_id.as_ref().unwrap();
        let client_secret = vars.xoauth_client_secret.as_ref().unwrap();
        let scope = vars.xoauth_scope.as_ref().unwrap();

        for _ in 0..vars.connect_retries {
            match HTTP_CLIENT
                .get()
                .unwrap()
                .post(url)
                .form(&RequestPayload {
                    client_id,
                    client_secret,
                    scope,
                    grant_type: "client_credentials",
                })
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let token = resp.json::<TokenResponse>().await?;
                        debug!("SMTP token: {token:?}");
                        return Ok(token);
                    }
                }
                Err(err) => {
                    error!(?err, "Error during SMTP XOAUTH2 token fetch");
                    time::sleep(Duration::from_secs(5)).await;
                }
            }
        }

        Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            "Could not fetch an SMTP OAuth token - retries exceeded",
        ))
    }
}
