use crate::rauthy_error::RauthyError;
use crate::token_set::{JwtRefreshClaims, OidcTokenSet};
use crate::{DangerAcceptInvalidCerts, RauthyHttpsOnly, RootCertificate, VERSION};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time;
use tracing::{debug, error, info, warn};

static TX_ACCESS_TOKEN: OnceLock<watch::Sender<String>> = std::sync::OnceLock::new();
static RX_ACCESS_TOKEN: OnceLock<watch::Receiver<String>> = std::sync::OnceLock::new();
static TX_ID_TOKEN: OnceLock<watch::Sender<Option<String>>> = std::sync::OnceLock::new();
static RX_ID_TOKEN: OnceLock<watch::Receiver<Option<String>>> = std::sync::OnceLock::new();

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
            .user_agent(format!("Rauthy OIDC Client v{VERSION}"));
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
                    format!("Rauthy request error - HTTP {status}: {body:?}")
                }
                Err(_) => {
                    format!("Rauthy request error  - HTTP {status}")
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
        let oidc_config_url = format!("{issuer}{append}");

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
                    return Err(RauthyError::Provider(Cow::from(format!("{err:?}"))));
                }

                OAuth2ErrorTypeResponse::ExpiredToken => {
                    return Err(RauthyError::Provider(Cow::from(format!("{err:?}"))));
                }

                // the others should not come up, only if the connection dies in between
                // or something like that
                _ => return Err(RauthyError::Provider(Cow::from(format!("{err:?}")))),
            }
        }
    }

    #[cfg(feature = "qrcode")]
    fn qr(&self) -> Result<qrcode::QrCode, RauthyError> {
        if let Some(uri) = &self.verification_uri_complete {
            Ok(qrcode::QrCode::new(uri)?)
        } else {
            Err(RauthyError::Provider(Cow::from(
                "did not receive a `verification_uri_complete`",
            )))
        }
    }

    #[cfg(feature = "qrcode")]
    pub fn qr_string(&self) -> Result<String, RauthyError> {
        use qrcode::render::unicode;

        let code = self.qr()?;
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        Ok(image)
    }

    #[cfg(feature = "qrcode")]
    pub fn qr_svg(&self) -> Result<String, RauthyError> {
        use qrcode::render::svg;

        let code = self.qr()?;
        let image = code
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        Ok(image)
    }
}

#[derive(Debug, Serialize)]
struct TokenRequest<'a> {
    // refresh_token
    grant_type: &'a str,
    client_id: &'a str,
    client_secret: &'a Option<String>,
    refresh_token: &'a str,
}

impl OidcTokenSet {
    pub async fn access_token() -> Result<String, RauthyError> {
        if let Some(rx) = RX_ACCESS_TOKEN.get() {
            Ok(rx.borrow().to_string())
        } else {
            Err(RauthyError::Init("You must spawn a refresh handler first"))
        }
    }

    pub async fn id_token() -> Result<Option<String>, RauthyError> {
        if let Some(rx) = RX_ID_TOKEN.get() {
            Ok(rx.borrow().clone())
        } else {
            Err(RauthyError::Init("You must spawn a refresh handler first"))
        }
    }

    pub async fn into_refresh_handler(
        self,
        client_id: String,
        client_secret: Option<String>,
    ) -> Result<(), RauthyError> {
        self.into_refresh_handler_with(
            client_id,
            client_secret,
            None,
            RauthyHttpsOnly::Yes,
            DangerAcceptInvalidCerts::No,
        )
        .await
    }

    pub async fn into_refresh_handler_with(
        self,
        client_id: String,
        client_secret: Option<String>,
        root_certificate: Option<RootCertificate>,
        https_only: RauthyHttpsOnly,
        danger_insecure: DangerAcceptInvalidCerts,
    ) -> Result<(), RauthyError> {
        if self.refresh_token.is_none() {
            return Err(RauthyError::Provider(Cow::from(
                "Misconfigured client - refresh_token is missing",
            )));
        }

        // this way of initializing makes it possible to restart a handler that went down
        // because of network issues
        if RX_ACCESS_TOKEN.get().is_none() {
            let (tx_access, rx_access) = watch::channel(self.access_token);
            let (tx_id, rx_id) = watch::channel(self.id_token);
            TX_ACCESS_TOKEN.set(tx_access).unwrap();
            RX_ACCESS_TOKEN.set(rx_access).unwrap();
            TX_ID_TOKEN.set(tx_id).unwrap();
            RX_ID_TOKEN.set(rx_id).unwrap();
        } else {
            TX_ACCESS_TOKEN.get().unwrap().send(self.access_token)?;
            TX_ID_TOKEN.get().unwrap().send(self.id_token)?;
        }

        // unwrap cannot panic - checked above already
        let refresh_token = self.refresh_token.unwrap();
        let refresh_claims =
            OidcTokenSet::danger_claims_unvalidated::<JwtRefreshClaims>(&refresh_token)?;
        let now = Utc::now().timestamp();

        if refresh_claims.exp < now {
            return Err(RauthyError::Token(Cow::from(
                "The refresh_token as already expired",
            )));
        }

        tokio::spawn(Self::refresh_handler(
            refresh_token,
            refresh_claims,
            client_id,
            client_secret,
            root_certificate,
            https_only,
            danger_insecure,
        ));

        Ok(())
    }

    async fn refresh_handler(
        mut refresh_token: String,
        mut refresh_claims: JwtRefreshClaims,
        client_id: String,
        client_secret: Option<String>,
        root_certificate: Option<RootCertificate>,
        https_only: RauthyHttpsOnly,
        danger_insecure: DangerAcceptInvalidCerts,
    ) {
        'main: loop {
            let sleep_for = refresh_claims.nbf - Utc::now().timestamp() + 1;
            if sleep_for >= 10 {
                time::sleep(Duration::from_secs(sleep_for as u64)).await;
            } else {
                warn!(
                    "refresh_token nbf is < 10 seconds -> you may want to increase access_token \
                    lifetime to not end up in a busy refresh loop"
                );
                // even if the token can be used right away, sleep for 1 second in any case
                // to avoid spamming Rauthy too much because of a misconfigured client
                time::sleep(Duration::from_secs(1)).await;
            }
            debug!("Refreshing token now");

            let token_url = format!("{}/oidc/token", refresh_claims.iss);
            let payload = TokenRequest {
                grant_type: "refresh_token",
                client_id: &client_id,
                client_secret: &client_secret,
                refresh_token: &refresh_token,
            };

            // this client builder cannot panic because it must have been working before
            let client = DeviceCode::build_client(
                root_certificate.clone(),
                https_only.clone(),
                danger_insecure.clone(),
            )
            .unwrap();

            let mut retries = 0;
            loop {
                match client.post(&token_url).form(&payload).send().await {
                    Ok(res) => {
                        let status = res.status();
                        if status.is_success() {
                            match res.json::<OidcTokenSet>().await {
                                Ok(ts) => {
                                    TX_ACCESS_TOKEN
                                        .get()
                                        .unwrap()
                                        .send(ts.access_token)
                                        .expect("internal send to not fail");
                                    TX_ID_TOKEN
                                        .get()
                                        .unwrap()
                                        .send(ts.id_token)
                                        .expect("internal send to not fail");

                                    if let Some(rt) = ts.refresh_token {
                                        info!("Rauthy token refreshed successfully");
                                        refresh_token = rt;
                                        refresh_claims =
                                            OidcTokenSet::danger_claims_unvalidated(&refresh_token)
                                                .unwrap();
                                        break;
                                    } else {
                                        warn!("Did not receive a new refresh_token from refresh - exiting refresh handler");
                                        break 'main;
                                    }
                                }
                                Err(err) => {
                                    error!("Error deserializing TokenSet: {:?}\nexiting refresh handler", err);
                                    break 'main;
                                }
                            }
                        } else {
                            let body = res.text().await.unwrap_or_default();
                            error!(
                                "Error during token refresh: {}: {}\nexiting refresh handler",
                                status, body
                            );
                            break 'main;
                        }
                    }
                    Err(err) => {
                        // we might get here, if the network is currently down -> retry
                        error!("Error refreshing token: {:?}", err);
                        retries += 1;
                        if retries > 10 {
                            error!("Refresh retries exceeded - exiting refresh handler");
                            break 'main;
                        }
                        time::sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        }
    }
}
