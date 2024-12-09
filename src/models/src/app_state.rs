use crate::database::DB;
use crate::email::EMail;
use crate::entity::atproto::{AtprotoClient, DnsTxtResolver};
use crate::events::event::Event;
use crate::events::ip_blacklist_handler::IpBlacklistReq;
use crate::events::listener::EventRouterMsg;
use crate::ListenScheme;
use atrium_identity::did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL};
use atrium_identity::handle::{AtprotoHandleResolver, AtprotoHandleResolverConfig};
use atrium_oauth_client::{
    AtprotoClientMetadata, AuthMethod, DefaultHttpClient, GrantType, KnownScope, OAuthClient,
    OAuthClientConfig, OAuthResolverConfig, Scope,
};
use rauthy_common::constants::PROXY_MODE;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};
use webauthn_rs::prelude::Url;
use webauthn_rs::Webauthn;

pub type DbPool = sqlx::PgPool;
pub type DbTxn<'a> = sqlx::Transaction<'a, sqlx::Postgres>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub public_url: String,
    pub argon2_params: argon2::Params,
    pub issuer: String,
    pub listen_addr: String,
    pub listen_scheme: ListenScheme,
    pub refresh_grace_time: u32,
    pub session_lifetime: u32,
    pub session_timeout: u32,
    pub ml_lt_pwd_first: u32,
    pub ml_lt_pwd_reset: u32,
    pub tx_email: mpsc::Sender<EMail>,
    pub tx_events: flume::Sender<Event>,
    pub tx_events_router: flume::Sender<EventRouterMsg>,
    pub tx_ip_blacklist: flume::Sender<IpBlacklistReq>,
    pub webauthn: Arc<Webauthn>,
    pub atproto: Arc<AtprotoClient>,
}

impl AppState {
    pub async fn new(
        tx_email: mpsc::Sender<EMail>,
        tx_events: flume::Sender<Event>,
        tx_events_router: flume::Sender<EventRouterMsg>,
        tx_ip_blacklist: flume::Sender<IpBlacklistReq>,
    ) -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        debug!("New AppState on {:?}", std::thread::current().id());

        // server listen address
        let listen_addr = env::var("LISTEN_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
        let listen_scheme = match env::var("LISTEN_SCHEME")
            .unwrap_or_else(|_| String::from("http_https"))
            .as_str()
        {
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
            "unix_http" => {
                info!("Listen URL: unix+http:{}", listen_addr);
                ListenScheme::UnixHttp
            }
            "unix_https" => {
                info!("Listen URL: unix+https:{}", listen_addr);
                ListenScheme::UnixHttps
            }
            _ => panic!(
                "'LISTEN_SCHEME' environment variable not correctly set (http | https | http_https)"
            ),
        };

        // public url
        let public_url = env::var("PUB_URL").expect("PUB_URL env var is not set");
        info!("Public URL: {}", public_url);

        // Argon2id config
        let argon2_m_cost = env::var("ARGON2_M_COST")
            .unwrap_or_else(|_| String::from("131072"))
            .parse::<u32>()
            .expect("Could not parse ARGON2_M_COST value");
        let argon2_t_cost = env::var("ARGON2_T_COST")
            .unwrap_or_else(|_| String::from("4"))
            .parse::<u32>()
            .expect("Could not parse ARGON2_T_COST value");
        let argon2_p_cost = env::var("ARGON2_P_COST")
            .unwrap_or_else(|_| String::from("8"))
            .parse::<u32>()
            .expect("Could not parse ARGON2_P_COST value");
        let argon2_params = argon2::Params::new(argon2_m_cost, argon2_t_cost, argon2_p_cost, None)
            .expect("Unable to build Argon2id params");
        debug!(
            "Argon2id Params: m_cost: {}, t_cost: {}, p_cost: {}",
            argon2_m_cost, argon2_t_cost, argon2_p_cost
        );

        let refresh_grace_time = env::var("REFRESH_TOKEN_GRACE_TIME")
            .unwrap_or_else(|_| String::from('5'))
            .parse::<u32>()
            .expect("Could not parse REFRESH_TOKEN_GRACE_TIME");

        let issuer_scheme = if matches!(
            listen_scheme,
            ListenScheme::HttpHttps | ListenScheme::Https | ListenScheme::UnixHttps
        ) || *PROXY_MODE
        {
            "https"
        } else {
            "http"
        };
        let issuer = format!("{}://{}/auth/v1", issuer_scheme, public_url);
        debug!("Issuer: {}", issuer);

        let session_lifetime = env::var("SESSION_LIFETIME")
            .unwrap_or_else(|_| String::from("14400"))
            .trim()
            .parse::<u32>()
            .expect("SESSION_LIFETIME cannot be parsed to u32 - bad format");
        let session_timeout = env::var("SESSION_TIMEOUT")
            .unwrap_or_else(|_| String::from("5400"))
            .trim()
            .parse::<u32>()
            .expect("SESSION_TIMEOUT cannot be parsed to u32 - bad format");

        let ml_lt_pwd_first = env::var("ML_LT_PWD_FIRST")
            .unwrap_or_else(|_| String::from("4320"))
            .trim()
            .parse::<u32>()
            .expect("ML_LT_PWD_FIRST cannot be parsed to u32 - bad format");
        let ml_lt_pwd_reset = env::var("ML_LT_PWD_RESET")
            .unwrap_or_else(|_| String::from("30"))
            .trim()
            .parse::<u32>()
            .expect("ML_LT_PWD_RESET cannot be parsed to u32 - bad format");

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

        let atproto = {
            let http_client = Arc::new(DefaultHttpClient::default());

            let listen_scheme = match listen_scheme {
                ListenScheme::Http | ListenScheme::UnixHttp => "http",
                ListenScheme::Https | ListenScheme::HttpHttps | ListenScheme::UnixHttps => "https",
            };

            let client_metadata = AtprotoClientMetadata {
                client_id: format!(
                    "{listen_scheme}://{public_url}/auth/v1/atproto/client_metadata"
                ),
                client_uri: format!("{listen_scheme}://{public_url}"),
                redirect_uris: vec![format!(
                    "{listen_scheme}://{public_url}/auth/v1/atproto/callback"
                )],
                token_endpoint_auth_method: AuthMethod::None,
                grant_types: vec![GrantType::AuthorizationCode],
                scopes: vec![Scope::Known(KnownScope::Atproto)],
                jwks_uri: None,
                token_endpoint_auth_signing_alg: None,
            };

            Arc::new(
                OAuthClient::new(OAuthClientConfig {
                    client_metadata: client_metadata.clone(),
                    keys: None,
                    resolver: OAuthResolverConfig {
                        did_resolver: CommonDidResolver::new(CommonDidResolverConfig {
                            plc_directory_url: DEFAULT_PLC_DIRECTORY_URL.to_string(),
                            http_client: http_client.clone(),
                        }),
                        handle_resolver: AtprotoHandleResolver::new(AtprotoHandleResolverConfig {
                            dns_txt_resolver: DnsTxtResolver::default(),
                            http_client: http_client.clone(),
                        }),
                        authorization_server_metadata: Default::default(),
                        protected_resource_metadata: Default::default(),
                    },
                    state_store: DB,
                    session_store: DB,
                })
                .expect("failed to initialize atproto client"),
            )
        };

        Ok(Self {
            public_url,
            argon2_params,
            issuer,
            listen_addr,
            listen_scheme,
            refresh_grace_time,
            session_lifetime,
            session_timeout,
            ml_lt_pwd_first,
            ml_lt_pwd_reset,
            tx_email,
            tx_events,
            tx_events_router,
            tx_ip_blacklist,
            webauthn,
            atproto,
        })
    }

    // pub async fn new_db_pool() -> anyhow::Result<DbPool> {
    //     let db_max_conn = env::var("DATABASE_MAX_CONN")
    //         .unwrap_or_else(|_| String::from("5"))
    //         .parse::<u32>()
    //         .expect("Error parsing DATABASE_MAX_CONN to u32");
    //
    //     let pool = {
    //         if *DB_TYPE == DbType::Sqlite {
    //             debug!("DATABASE_URL: {}", *DATABASE_URL);
    //
    //             let msg = r#"
    // You are trying to connect to a SQLite instance with the 'Postgres'
    // version of Rauthy. You need to either change to a SQLite database or use the '*-lite'
    // container image of Rauthy."#;
    //             error!("{msg}");
    //             panic!("{msg}");
    //         }
    //
    //         info!("Trying to connect to Postgres instance");
    //         let pool = Self::connect_postgres(&DATABASE_URL, db_max_conn).await?;
    //         info!("Database Connection established");
    //
    //         debug!("Migrating data from ../../migrations/postgres");
    //         sqlx::migrate!("../../migrations/postgres")
    //             .run(&pool)
    //             .await?;
    //
    //         pool
    //     };
    //
    //     Ok(pool)
    // }

    // pub async fn connect_sqlite(
    //     addr: &str,
    //     max_conn: u32,
    //     // migration_only: bool,
    // ) -> anyhow::Result<sqlx::SqlitePool> {
    //     let opts = sqlx::sqlite::SqliteConnectOptions::from_str(addr)?
    //         .create_if_missing(true)
    //         .busy_timeout(Duration::from_millis(100))
    //         .foreign_keys(true)
    //         .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
    //         .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
    //         .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    //
    //     let pool = PoolOptions::new()
    //         .min_connections(2)
    //         .max_connections(max_conn)
    //         .acquire_timeout(Duration::from_secs(10))
    //         .connect_with(opts)
    //         .await
    //         .context("failed to connect to sqlite")?;
    //
    //     info!("Database Connection Pool created successfully");
    //
    //     Ok(pool)
    // }
    //
    // pub async fn connect_postgres(addr: &str, max_conn: u32) -> anyhow::Result<sqlx::PgPool> {
    //     let opts = sqlx::postgres::PgConnectOptions::from_str(addr)?
    //         .log_slow_statements(LevelFilter::Debug, Duration::from_secs(3));
    //     let pool = PoolOptions::new()
    //         .min_connections(2)
    //         .max_connections(max_conn)
    //         .acquire_timeout(Duration::from_secs(10))
    //         .connect_with(opts)
    //         .await
    //         .context("failed to connect to postgres")?;
    //
    //     info!("Database Connection Pool created successfully");
    //
    //     Ok(pool)
    // }
}
