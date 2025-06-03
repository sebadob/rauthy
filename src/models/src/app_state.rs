use crate::ListenScheme;
use crate::email::EMail;
use crate::events::event::Event;
use crate::events::listener::EventRouterMsg;
use rauthy_common::constants::PROXY_MODE;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};
use webauthn_rs::Webauthn;
use webauthn_rs::prelude::Url;

// TODO the cleaned approach would probably be to move the AppState into a `LazyLock`
// and push all constants that are needed all the time and do not n eed lazy init on their own
// into it. This approach would be quite a bit cleaner.
#[derive(Debug, Clone)]
pub struct AppState {
    pub public_url: String,
    pub argon2_params: argon2::Params,
    pub issuer: String,
    pub listen_addr: String,
    pub listen_scheme: ListenScheme,
    pub refresh_grace_time: u32,
    pub ml_lt_pwd_first: u32,
    pub ml_lt_pwd_reset: u32,
    pub tx_email: mpsc::Sender<EMail>,
    pub tx_events: flume::Sender<Event>,
    pub tx_events_router: flume::Sender<EventRouterMsg>,
    pub webauthn: Arc<Webauthn>,
}

impl AppState {
    pub async fn new(
        tx_email: mpsc::Sender<EMail>,
        tx_events: flume::Sender<Event>,
        tx_events_router: flume::Sender<EventRouterMsg>,
    ) -> Result<Self, Box<dyn Error>> {
        let listen_addr = env::var("LISTEN_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
        let listen_scheme = match env::var("LISTEN_SCHEME").as_deref().unwrap_or("http_https") {
            "http" => {
                let port = env::var("LISTEN_PORT_HTTP").unwrap_or_else(|_| "8080".to_string());
                info!("Listen URL: http://{}:{}", listen_addr, port);
                ListenScheme::Http
            }
            "https" => {
                let port = env::var("LISTEN_PORT_HTTPS").unwrap_or_else(|_| "8443".to_string());
                info!("Listen URL: https://{}:{}", listen_addr, port);
                ListenScheme::Https
            }
            "http_https" => {
                let port_http = env::var("LISTEN_PORT_HTTP").unwrap_or_else(|_| "8080".to_string());
                let port_https =
                    env::var("LISTEN_PORT_HTTPS").unwrap_or_else(|_| "8443".to_string());
                let port = format!("{{{}|{}}}", port_http, port_https);
                info!("Listen URL: {{http|https}}://{}:{}", listen_addr, port);
                ListenScheme::HttpHttps
            }
            #[cfg(not(target_os = "windows"))]
            "unix_http" => {
                info!("Listen URL: unix+http:{}", listen_addr);
                ListenScheme::UnixHttp
            }
            #[cfg(not(target_os = "windows"))]
            "unix_https" => {
                info!("Listen URL: unix+https:{}", listen_addr);
                ListenScheme::UnixHttps
            }
            _ => panic!(
                "'LISTEN_SCHEME' environment variable not correctly set (http | https | http_https)"
            ),
        };

        let public_url = env::var("PUB_URL").expect("PUB_URL env var is not set");
        info!("Public URL: {}", public_url);

        let argon2_m_cost = env::var("ARGON2_M_COST")
            .as_deref()
            .unwrap_or("131072")
            .parse::<u32>()
            .expect("Could not parse ARGON2_M_COST value");
        let argon2_t_cost = env::var("ARGON2_T_COST")
            .as_deref()
            .unwrap_or("4")
            .parse::<u32>()
            .expect("Could not parse ARGON2_T_COST value");
        let argon2_p_cost = env::var("ARGON2_P_COST")
            .as_deref()
            .unwrap_or("8")
            .parse::<u32>()
            .expect("Could not parse ARGON2_P_COST value");
        let argon2_params = argon2::Params::new(argon2_m_cost, argon2_t_cost, argon2_p_cost, None)
            .expect("Unable to build Argon2id params");
        debug!(
            "Argon2id Params: m_cost: {}, t_cost: {}, p_cost: {}",
            argon2_m_cost, argon2_t_cost, argon2_p_cost
        );

        let refresh_grace_time = env::var("REFRESH_TOKEN_GRACE_TIME")
            .as_deref()
            .unwrap_or("5")
            .parse::<u32>()
            .expect("Could not parse REFRESH_TOKEN_GRACE_TIME");

        #[cfg(target_os = "windows")]
        let is_https =
            matches!(listen_scheme, ListenScheme::HttpHttps | ListenScheme::Https) || *PROXY_MODE;
        #[cfg(not(target_os = "windows"))]
        let is_https = matches!(
            listen_scheme,
            ListenScheme::HttpHttps | ListenScheme::Https | ListenScheme::UnixHttps
        ) || *PROXY_MODE;
        let issuer_scheme = if is_https { "https" } else { "http" };

        let issuer = format!("{}://{}/auth/v1", issuer_scheme, public_url);
        debug!("Issuer: {}", issuer);

        let ml_lt_pwd_first = env::var("ML_LT_PWD_FIRST")
            .as_deref()
            .unwrap_or("4320")
            .trim()
            .parse::<u32>()
            .expect("ML_LT_PWD_FIRST cannot be parsed as u32");
        let ml_lt_pwd_reset = env::var("ML_LT_PWD_RESET")
            .as_deref()
            .unwrap_or("30")
            .trim()
            .parse::<u32>()
            .expect("ML_LT_PWD_RESET cannot be parsed as u32");

        let rp_id = env::var("RP_ID").unwrap_or_else(|_| String::from("localhost"));
        let rp_origin_str =
            env::var("RP_ORIGIN").unwrap_or_else(|_| String::from("http://localhost:8080"));
        let rp_origin = Url::parse(&rp_origin_str).expect("Cannot parse RP_ORIGIN to URL");
        let rp_name = env::var("RP_NAME").unwrap_or_else(|_| String::from("Rauthy Webauthn"));
        let builder = webauthn_rs::WebauthnBuilder::new(&rp_id, &rp_origin)
            .expect("Invalid configuration")
            // Set a "nice" relying party name. Has no security properties - may be changed in the future.
            .rp_name(&rp_name);
        let webauthn = Arc::new(builder.build().expect("Invalid configuration"));

        Ok(Self {
            public_url,
            argon2_params,
            issuer,
            listen_addr,
            listen_scheme,
            refresh_grace_time,
            ml_lt_pwd_first,
            ml_lt_pwd_reset,
            tx_email,
            tx_events,
            tx_events_router,
            webauthn,
        })
    }
}
