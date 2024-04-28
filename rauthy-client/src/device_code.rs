use crate::rauthy_error::RauthyError;
use crate::token_set::OidcTokenSet;
use crate::{DangerAcceptInvalidCerts, RauthyHttpsOnly, RootCertificate, VERSION};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::time::Duration;

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
struct DeviceGrantTokenRequest<'a> {
    pub client_id: &'a str,
    pub client_secret: Option<&'a str>,
    pub device_code: &'a str,
}

#[derive(Debug, Deserialize)]
struct MetaResponse {
    pub device_authorization_endpoint: String,
    pub token_endpoint: String,
}

pub struct DeviceCode {
    client: reqwest::Client,
    token_endpoint: String,
    token_endpoint_payload: String,

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
    pub async fn request(
        issuer: &str,
        client_id: &str,
        client_secret: Option<&str>,
        scope: Option<&str>,
    ) -> Result<Self, RauthyError> {
        Self::request_with(
            issuer,
            client_id,
            client_secret,
            scope,
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
        client_id: &str,
        client_secret: Option<&str>,
        scope: Option<&str>,
        root_certificate: Option<RootCertificate>,
        https_only: RauthyHttpsOnly,
        danger_insecure: DangerAcceptInvalidCerts,
    ) -> Result<Self, RauthyError> {
        // fetch metadata endpoint
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
                    client_id,
                    client_secret,
                    scope,
                }),
        )
        .await?;
        let expires = Utc::now().add(chrono::Duration::seconds(device_code.expires_in as i64));
        let token_endpoint_payload = serde_json::to_string(&DeviceGrantTokenRequest {
            client_id,
            client_secret,
            device_code: &device_code.device_code,
        })
        .expect("DeviceGrantTokenRequest to always succeed");

        Ok(DeviceCode {
            client,
            token_endpoint: meta.token_endpoint,
            token_endpoint_payload,
            expires,
            user_code: device_code.user_code,
            verification_uri: device_code.verification_uri,
            verification_uri_complete: device_code.verification_uri_complete,
        })
    }

    /// With a valid `device_code`, continuously poll the Rauthy instance and wait
    /// for user verification of your request, to get an OIDC Token Set.
    pub async fn wait_for_token(&self) -> Result<OidcTokenSet, RauthyError> {
        todo!()
    }
}
