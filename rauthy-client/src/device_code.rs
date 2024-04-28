use crate::rauthy_error::RauthyError;
use crate::token_set::OidcTokenSet;
use crate::{DangerAcceptInvalidCerts, RauthyHttpsOnly, RootCertificate, VERSION};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::time::Duration;
use tracing::{debug, info, warn};

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    pub expires_in: u16,
    pub interval: Option<u8>,
}

#[derive(Serialize)]
struct DeviceGrantRequest<'a> {
    pub client_id: &'a str,
    pub client_secret: Option<&'a str>,
    pub scope: Option<&'a str>,
}

#[derive(Serialize)]
struct DeviceGrantTokenRequest {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub device_code: String,
    pub grant_type: &'static str,
}

#[derive(Debug, Deserialize)]
struct MetaResponse {
    pub device_authorization_endpoint: String,
    pub token_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2ErrorResponse<'a> {
    pub error: OAuth2ErrorTypeResponse,
    pub error_description: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuth2ErrorTypeResponse {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
    // specific to the device grant
    AuthorizationPending,
    SlowDown,
    AccessDenied,
    ExpiredToken,
}

pub struct DeviceCode {
    client: reqwest::Client,
    token_endpoint: String,
    token_endpoint_payload: DeviceGrantTokenRequest,
    // token_endpoint_payload: String,
    interval: u64,

    pub expires: DateTime<Utc>,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
}

impl Debug for DeviceCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeviceCode {{ token_endpoint: {}, token_endpoint_payload: <hidden>, expires: {}, \
            user_code: {}, verification_uri: {}, verification_uri_complete: {:?} }}",
            self.token_endpoint,
            self.expires,
            self.user_code,
            self.verification_uri,
            self.verification_uri_complete,
        )
    }
}

impl Display for DeviceCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
    Please visit {} and enter your User Code: {}
"#,
            self.verification_uri, self.user_code
        )
    }
}

impl DeviceCode {
    fn build_client(
        root_certificate: Option<RootCertificate>,
        https_only: RauthyHttpsOnly,
        danger_insecure: DangerAcceptInvalidCerts,
    ) -> Result<reqwest::Client, RauthyError> {
        let builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .https_only(https_only.bool())
            .danger_accept_invalid_certs(danger_insecure.bool())
            .user_agent(format!("Rauthy OIDC Client v{}", VERSION))
            .timeout(Duration::from_secs(10));
        if let Some(root) = root_certificate {
            Ok(builder.add_root_certificate(root).build()?)
        } else {
            Ok(builder.build()?)
        }
    }

    async fn fetch<T>(req: reqwest::RequestBuilder) -> Result<T, RauthyError>
    where
        T: Debug + for<'a> Deserialize<'a>,
    {
        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status().as_u16();
            let err = match res.text().await {
                Ok(body) => {
                    format!("Rauthy request error - HTTP {}: {:?}", status, body)
                }
                Err(_) => {
                    format!("Rauthy request error  - HTTP {}", status)
                }
            };
            return Err(RauthyError::Request(Cow::from(err)));
        }
        Ok(res.json::<T>().await?)
    }

    /// Request a `device_code` from your Rauthy instance.
    /// This code can then be used in exchange for an OIDC Token Set.
    /// Clients requesting `device_code`'s are typically public. If you have a confidential
    /// client or need additional configuration, use `DeviceCode::request_with()`.
    pub async fn request(issuer: &str, client_id: String) -> Result<Self, RauthyError> {
        Self::request_with(
            issuer,
            client_id,
            None,
            None,
            None,
            RauthyHttpsOnly::Yes,
            DangerAcceptInvalidCerts::No,
        )
        .await
    }

    /// Request a `device_code` from your Rauthy instance.
    /// This code can then be used in exchange for an OIDC Token Set.
    pub async fn request_with(
        issuer: &str,
        client_id: String,
        client_secret: Option<String>,
        scope: Option<&str>,
        root_certificate: Option<RootCertificate>,
        https_only: RauthyHttpsOnly,
        danger_insecure: DangerAcceptInvalidCerts,
    ) -> Result<Self, RauthyError> {
        let append = if issuer.ends_with('/') {
            ".well-known/openid-configuration"
        } else {
            "/.well-known/openid-configuration"
        };
        let oidc_config_url = format!("{}{}", issuer, append);

        let client = Self::build_client(root_certificate, https_only, danger_insecure)?;
        let meta = Self::fetch::<MetaResponse>(client.get(oidc_config_url)).await?;
        let device_code = Self::fetch::<DeviceCodeResponse>(
            client
                .post(meta.device_authorization_endpoint)
                .form(&DeviceGrantRequest {
                    client_id: &client_id,
                    client_secret: client_secret.as_deref(),
                    scope,
                }),
        )
        .await?;
        let expires = Utc::now().add(chrono::Duration::seconds(device_code.expires_in as i64));
        let token_endpoint_payload = DeviceGrantTokenRequest {
            client_id,
            client_secret,
            device_code: device_code.device_code,
            grant_type: "urn:ietf:params:oauth:grant-type:device_code",
        };

        Ok(DeviceCode {
            client,
            token_endpoint: meta.token_endpoint,
            token_endpoint_payload,
            interval: device_code.interval.unwrap_or(5) as u64,
            expires,
            user_code: device_code.user_code,
            verification_uri: device_code.verification_uri,
            verification_uri_complete: device_code.verification_uri_complete,
        })
    }

    /// With a valid `device_code`, continuously poll the Rauthy instance and wait
    /// for user verification of your request, to get an OIDC Token Set.
    pub async fn wait_for_token(&mut self) -> Result<OidcTokenSet, RauthyError> {
        let mut wait_for = self.interval;

        loop {
            tokio::time::sleep(Duration::from_secs(wait_for)).await;

            let res = self
                .client
                .post(&self.token_endpoint)
                .form(&self.token_endpoint_payload)
                .send()
                .await?;

            if res.status().is_success() {
                let ts = res.json::<OidcTokenSet>().await?;
                info!("Success - received an OIDC TokenSet");
                return Ok(ts);
            }

            let err = res.json::<OAuth2ErrorResponse>().await?;
            match err.error {
                OAuth2ErrorTypeResponse::AuthorizationPending => {
                    debug!("Authorization Pending - awaiting user verification");
                }

                // this should not happen with Rauthy -> should always be just 5 seconds
                OAuth2ErrorTypeResponse::SlowDown => {
                    warn!("Received a `slow_down` - doubling token fetch interval");
                    wait_for *= 2;
                }

                OAuth2ErrorTypeResponse::AccessDenied => {
                    return Err(RauthyError::Provider(Cow::from(format!("{:?}", err))));
                }

                OAuth2ErrorTypeResponse::ExpiredToken => {
                    return Err(RauthyError::Provider(Cow::from(format!("{:?}", err))));
                }

                // the others should not come up, only if the connection dies in between
                // or something like that
                _ => return Err(RauthyError::Provider(Cow::from(format!("{:?}", err)))),
            }
        }
    }
}
