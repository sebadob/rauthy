use crate::database::DB;
use crate::email::mailer_microsoft_graph::sender_microsoft_graph;
use crate::email::smtp_oauth_token::SmtpOauthToken;
use crate::rauthy_config::RauthyConfig;
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::{authentication, client};
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor, message};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct EMail {
    pub recipient_name: String,
    pub address: String,
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum SmtpConnMode {
    Default,
    XOauth2,
    MicrosoftGraph,
    Test,
}

impl From<&str> for SmtpConnMode {
    fn from(s: &str) -> Self {
        match s {
            "default" => Self::Default,
            "xoauth2" => Self::XOauth2,
            "microsoft_graph" => Self::MicrosoftGraph,
            "test" => Self::Test,
            _ => panic!("Invalid smtp_conn_mode, expected one of: default xoauth2 microsoft_graph"),
        }
    }
}

pub async fn sender(rx: mpsc::Receiver<EMail>) {
    debug!("E-Mail sender started");

    let vars = &RauthyConfig::get().vars.email;
    let url = &vars.smtp_url;
    if url.is_none() && vars.smtp_conn_mode != SmtpConnMode::MicrosoftGraph {
        sender_test_debug(rx).await;
    } else {
        match vars.smtp_conn_mode {
            SmtpConnMode::Default | SmtpConnMode::XOauth2 => {
                sender_default_smtp(url.as_deref().unwrap(), rx).await
            }
            SmtpConnMode::MicrosoftGraph => sender_microsoft_graph(rx).await,
            SmtpConnMode::Test => sender_test_debug(rx).await,
        }
    }
}

async fn sender_test_debug(mut rx: mpsc::Receiver<EMail>) {
    warn!("SMTP_URL is not configured or test mode is set, cannot send out any E-Mails!");

    loop {
        if let Some(email) = rx.recv().await {
            debug!("New E-Mail for address: {}", email.address);
        } else {
            warn!("Received 'None' in email 'sender' - exiting");
            return;
        }
    }
}

async fn sender_default_smtp(smtp_url: &str, mut rx: mpsc::Receiver<EMail>) {
    let vars = &RauthyConfig::get().vars.email;
    let from: message::Mailbox = vars
        .smtp_from
        .as_ref()
        .parse()
        .expect("SMTP_FROM could not be parsed correctly");

    let mut mailer = create_mailer(smtp_url).await;
    loop {
        debug!("Listening for incoming send E-Mail requests");
        if let Some(req) = rx.recv().await {
            debug!("New E-Mail for address: {:?}", req.address);

            let to = format!("{} <{}>", req.recipient_name, req.address);

            let email = if req.html.is_some() && req.text.is_some() {
                #[allow(clippy::unnecessary_unwrap)]
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .multipart(MultiPart::alternative_plain_html(
                        req.text.unwrap(),
                        req.html.unwrap(),
                    ))
            } else if let Some(html) = req.html {
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .singlepart(SinglePart::html(html))
            } else if let Some(text) = req.text {
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .singlepart(SinglePart::plain(text))
            } else {
                warn!("Sending E-Mail with empty body!");
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .singlepart(SinglePart::plain(String::default()))
            };

            match email {
                Ok(message) => {
                    match mailer.send(message.clone()).await {
                        Ok(_) => {
                            info!("E-Mail to '{}' sent successfully!", req.address);
                            continue;
                        }
                        Err(err) => {
                            error!(error = ?err, "Could not send E-Mail - establishing new connection and retry");
                        }
                    }

                    // short timeout if sending fails, maybe the network just had a short hiccup
                    tokio::time::sleep(Duration::from_millis(500)).await;

                    // Only try to recreate the connection if we use XOAUTH2, because it uses
                    // expiring tokens for authentication. Normal SMTP connections should handle
                    // reconnects automatically under the hood.
                    if vars.smtp_conn_mode == SmtpConnMode::XOauth2 {
                        mailer = create_mailer(smtp_url).await;
                    }

                    match mailer.send(message.clone()).await {
                        Ok(_) => {
                            info!("E-Mail to '{}' sent successfully after retry!", req.address);
                        }
                        Err(err) => {
                            // we want to panic if multiple sends fail so emails don't get lost
                            // silently
                            panic!("Could not send E-Mail even after retrying: {err:?}");
                        }
                    }
                }
                Err(err) => {
                    // this should never happen
                    error!("Error building the E-Mail to '{}': {:?}", req.address, err);
                }
            }
        } else {
            warn!("Received 'None' in email 'sender' - exiting");
            break;
        }
    }
}

/// Connects to SMTP.
///
/// # Panics
///
/// If the connection is not possible after retries were exceeded.
async fn create_mailer(smtp_url: &str) -> AsyncSmtpTransport<Tokio1Executor> {
    let vars = &RauthyConfig::get().vars.email;

    let mut conn = if vars.danger_insecure {
        conn_test_smtp_insecure(smtp_url, vars.smtp_port).await
    } else {
        connect_test_smtp(smtp_url, vars.smtp_port).await
    };

    let mut retries = 0;
    while let Err(err) = conn {
        error!(?err);

        if retries >= vars.connect_retries {
            // do a graceful shutdown of the DB before `panic`king
            if RauthyConfig::get().is_ha_cluster {
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            DB::hql().shutdown().await.unwrap();

            panic!("SMTP connection retries exceeded");
        }
        retries += 1;
        tokio::time::sleep(Duration::from_secs(5)).await;

        conn = if vars.danger_insecure {
            conn_test_smtp_insecure(smtp_url, vars.smtp_port).await
        } else {
            connect_test_smtp(smtp_url, vars.smtp_port).await
        }
    }
    conn.unwrap()
}

#[tracing::instrument(level = "debug")]
async fn connect_test_smtp(
    smtp_url: &str,
    smtp_port: Option<u16>,
) -> Result<AsyncSmtpTransport<lettre::Tokio1Executor>, ErrorResponse> {
    let vars = &RauthyConfig::get().vars.email;
    let username = vars
        .smtp_username
        .as_deref()
        .expect("SMTP_USERNAME is not set")
        .trim()
        .to_string();

    let mut mechanisms = Vec::with_capacity(2);
    let creds = if vars.smtp_conn_mode == SmtpConnMode::XOauth2 {
        mechanisms.push(Mechanism::Xoauth2);

        let token = SmtpOauthToken::get()
            .await
            .expect("Could not retrieve a `client_credntials` token for SMTP XOAUTH2");
        authentication::Credentials::new(username, token.access_token)
    } else {
        mechanisms.push(Mechanism::Plain);
        mechanisms.push(Mechanism::Login);

        let password = vars
            .smtp_password
            .as_deref()
            .expect("SMTP_PASSWORD is not set")
            .trim()
            .to_string();
        authentication::Credentials::new(username, password)
    };

    let mut builder = if vars.starttls_only {
        AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(smtp_url)
            .expect("Connection Error with 'SMTP_URL'")
    } else {
        AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(smtp_url)
            .expect("Connection Error with 'SMTP_URL'")
    };

    if let Some(port) = smtp_port {
        builder = builder.port(port);
    }

    if let Some(root_ca) = &RauthyConfig::get().vars.email.root_ca {
        let cert = client::Certificate::from_pem(root_ca.as_bytes())
            .expect("Invalid `email.root_ca` for SMTP connections");

        let params = client::TlsParameters::builder(smtp_url.to_string())
            .add_root_certificate(cert)
            .build_rustls()
            .expect("Cannot build TLS parameters with custom `email.root_ca`");

        builder = builder.tls(client::Tls::Required(params));
    }

    let conn = builder
        .authentication(mechanisms.clone())
        .credentials(creds.clone())
        .timeout(Some(Duration::from_secs(10)))
        .build();
    info!("SMTP connection opened");

    if vars.starttls_only {
        match conn.test_connection().await {
            Ok(true) => {
                info!(smtp_url, "Successfully connected via STARTTLS");
                return Ok(conn);
            }
            Ok(false) => {
                error!(smtp_url, "Could not connect via STARTTLS");
            }
            Err(err) => {
                error!(
                    smtp_url,
                    ?err,
                    "Could not connect via STARTTLS. Check credentials",
                );
            }
        }
    } else {
        match conn.test_connection().await {
            Ok(true) => {
                info!("Successfully connected to {smtp_url} via TLS");
                return Ok(conn);
            }
            Ok(false) => {
                error!("Could not connect to {} via TLS.", smtp_url,);
            }
            Err(err) => {
                error!(
                    ?err,
                    "Could not connect to {smtp_url} via TLS. Check credentials"
                );
            }
        }
    }

    Err(ErrorResponse::new(
        ErrorResponseType::Internal,
        format!("Could not connect to {smtp_url} - neither TLS nor STARTTLS worked"),
    ))
}

async fn conn_test_smtp_insecure(
    smtp_url: &str,
    smtp_port: Option<u16>,
) -> Result<AsyncSmtpTransport<lettre::Tokio1Executor>, ErrorResponse> {
    let port = smtp_port.unwrap_or(1025);

    let conn = AsyncSmtpTransport::<lettre::Tokio1Executor>::builder_dangerous(smtp_url)
        .port(port)
        .build();
    match conn.test_connection().await {
        Ok(true) => {
            info!(
                smtp_url,
                port, "Successfully connected to INSECURE SMTP relay ",
            );
            Ok(conn)
        }
        Ok(false) => {
            error!(smtp_url, port, "Could not connect to insecure SMTP relay",);
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Could not connect to localhost SMTP relay",
            ))
        }
        Err(err) => {
            error!(
                smtp_url,
                port,
                ?err,
                "Could not connect to insecure SMTP relay",
            );
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Could not connect to localhost SMTP relay",
            ))
        }
    }
}
