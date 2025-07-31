use crate::database::DB;
use crate::rauthy_config::RauthyConfig;
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication;
use lettre::{AsyncSmtpTransport, AsyncTransport, message};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct EMail {
    pub recipient_name: String,
    pub address: String,
    pub subject: String,
    pub text: String,
    pub html: Option<String>,
}

pub async fn sender(mut rx: mpsc::Receiver<EMail>, test_mode: bool) {
    debug!("E-Mail sender started");

    // to make the integration tests not panic, results are taken and just thrown away
    // not the nicest approach for now, but it works
    let vars = &RauthyConfig::get().vars.email;
    let url = &vars.smtp_url;
    if test_mode || url.is_none() {
        if url.is_none() {
            error!("SMTP_URL is not configured, cannot send out any E-Mails!");
        }

        loop {
            let req = rx.recv().await;
            if req.is_some() {
                debug!(
                    "New E-Mail for address: {:?}",
                    req.as_ref().unwrap().address
                );
            } else {
                warn!("Received 'None' in email 'sender' - exiting");
                return;
            }
        }
    }

    let mailer = {
        let smtp_url = url.as_deref().unwrap();

        let mut retries = 0;

        let mut conn = if vars.danger_insecure {
            conn_test_smtp_insecure(smtp_url, vars.smtp_port).await
        } else {
            connect_test_smtp(smtp_url, vars.smtp_port).await
        };

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
    };

    let from: message::Mailbox = RauthyConfig::get()
        .vars
        .email
        .smtp_from
        .as_ref()
        .parse()
        .expect("SMTP_FROM could not be parsed correctly");

    loop {
        debug!("Listening for incoming send E-Mail requests");
        if let Some(req) = rx.recv().await {
            debug!("New E-Mail for address: {:?}", req.address);

            let to = format!("{} <{}>", req.recipient_name, req.address);

            let email = if let Some(html) = req.html {
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .multipart(MultiPart::alternative_plain_html(req.text, html))
            } else {
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .singlepart(SinglePart::plain(req.text))
            };

            match email {
                Ok(addr) => match mailer.send(addr).await {
                    Ok(_) => info!("E-Mail to '{}' sent successfully!", req.address),
                    Err(e) => error!(error = ?e, "Could not send E-Mail"),
                },
                Err(_) => error!("Error building the E-Mail to '{}'", req.address),
            }
        } else {
            warn!("Received 'None' in email 'sender' - exiting");
            break;
        }
    }
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
    let password = vars
        .smtp_password
        .as_deref()
        .expect("SMTP_PASSWORD is not set")
        .trim()
        .to_string();

    let creds = authentication::Credentials::new(username, password);

    // always try fully wrapped TLS first
    let mut builder = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(smtp_url)
        .expect("Connection Error with 'SMTP_URL'");
    if let Some(port) = smtp_port {
        builder = builder.port(port);
    }
    let mut conn = builder
        .credentials(creds.clone())
        .timeout(Some(Duration::from_secs(10)))
        .build();

    match conn.test_connection().await {
        Ok(true) => {
            info!("Successfully connected to {smtp_url} via TLS");
            return Ok(conn);
        }
        Ok(false) => {
            warn!(
                "Could not connect to {} via TLS. Trying downgrade to STARTTLS",
                smtp_url,
            );
        }
        Err(err) => {
            warn!(?err, "Could not connect to {smtp_url} via TLS");
        }
    }

    // only if full TLS fails, try STARTTLS
    builder = AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(smtp_url)
        .expect("Connection Error with 'SMTP_URL'");
    if let Some(port) = smtp_port {
        builder = builder.port(port);
    }
    conn = builder
        .credentials(creds)
        .timeout(Some(Duration::from_secs(10)))
        .build();

    match conn.test_connection().await {
        Ok(true) => {
            info!(smtp_url, "Successfully connected via STARTTLS");
            return Ok(conn);
        }
        Ok(false) => {
            error!(smtp_url, "Could not connect via STARTTLS either");
        }
        Err(err) => {
            warn!(smtp_url, ?err, "Could not connect via STARTTLS either",);
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
            warn!(
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
