use crate::email::EMail;
use crate::entity::db_version::DbVersion;
use crate::events::event::Event;
use crate::events::ip_blacklist_handler::IpBlacklistReq;
use crate::events::listener::EventRouterMsg;
use crate::migration::db_migrate;
use crate::migration::db_migrate::migrate_init_prod;
use crate::migration::db_migrate_dev::migrate_dev_data;
use crate::ListenScheme;
use anyhow::Context;
use argon2::Params;
use hiqlite::NodeConfig;
use rauthy_common::constants::{DATABASE_URL, DB_TYPE, DEV_MODE, PROXY_MODE};
use rauthy_common::DbType;
use sqlx::pool::PoolOptions;
use sqlx::ConnectOptions;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::log::LevelFilter;
use tracing::{debug, error, info, warn};
use webauthn_rs::prelude::Url;
use webauthn_rs::Webauthn;

#[cfg(feature = "postgres")]
pub type DbPool = sqlx::PgPool;
#[cfg(not(feature = "postgres"))]
pub type DbPool = sqlx::SqlitePool;

#[cfg(feature = "postgres")]
pub type DbTxn<'a> = sqlx::Transaction<'a, sqlx::Postgres>;
#[cfg(not(feature = "postgres"))]
pub type DbTxn<'a> = sqlx::Transaction<'a, sqlx::Sqlite>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DbPool,
    pub public_url: String,
    pub argon2_params: Argon2Params,
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
        let params = argon2::Params::new(argon2_m_cost, argon2_t_cost, argon2_p_cost, None)
            .expect("Unable to build Argon2id params");
        debug!(
            "Argon2id Params: m_cost: {}, t_cost: {}, p_cost: {}",
            argon2_m_cost, argon2_t_cost, argon2_p_cost
        );
        let argon2_params = Argon2Params { params };

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

        let db = Self::new_db_pool(&argon2_params.params, &issuer).await?;

        Ok(Self {
            db,
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
        })
    }

    pub async fn new_db_pool(argon2_params: &Params, issuer: &str) -> anyhow::Result<DbPool> {
        let db_max_conn = env::var("DATABASE_MAX_CONN")
            .unwrap_or_else(|_| String::from("5"))
            .parse::<u32>()
            .expect("Error parsing DATABASE_MAX_CONN to u32");

        #[cfg(feature = "postgres")]
        let pool = {
            if *DB_TYPE == DbType::Sqlite {
                debug!("DATABASE_URL: {}", *DATABASE_URL);

                let msg = r#"
    You are trying to connect to a SQLite instance with the 'Postgres'
    version of Rauthy. You need to either change to a SQLite database or use the '*-lite'
    container image of Rauthy."#;
                error!("{msg}");
                panic!("{msg}");
            }

            info!("Trying to connect to Postgres instance");
            let pool = Self::connect_postgres(&DATABASE_URL, db_max_conn).await?;
            info!("Database Connection established");

            debug!("Migrating data from ../../migrations/postgres");
            sqlx::migrate!("../../migrations/postgres")
                .run(&pool)
                .await?;

            pool
        };

        #[cfg(not(feature = "postgres"))]
        let pool = {
            if *DB_TYPE == DbType::Postgres {
                let msg = r#"
    You are trying to connect to a Postgres instance with the 'SQLite'
    version of Rauthy. You need to either change to a Postgres database or use the default
    Postgres container image of Rauthy."#;
                error!("{msg}");
                panic!("{msg}");
            }

            let pool = Self::connect_sqlite(&DATABASE_URL, db_max_conn).await?;
            if DATABASE_URL.ends_with(":memory:") {
                info!("Using in-memory SQLite");
            } else {
                info!("Using on-disk SQLite");
            }

            debug!("Migrating data from ../../migrations/sqlite");
            sqlx::migrate!("../../migrations/sqlite").run(&pool).await?;

            pool
        };

        // before we do any db migrations, we need to check the current DB version
        // for compatibility
        let db_version = DbVersion::check_app_version(&pool)
            .await
            .map_err(|err| anyhow::Error::msg(err.message))?;

        // migrate DB data
        if !*DEV_MODE {
            migrate_init_prod(&pool, argon2_params.clone(), issuer)
                .await
                .map_err(|err| anyhow::Error::msg(err.message))?;
        }

        if let Ok(from) = env::var("MIGRATE_DB_FROM") {
            if is_multi_replica_deployment() {
                error!(
                    r#"
    You cannot use 'MIGRATE_DB_FROM' with a multi replica deployment.
    You need to change your config to only have a single node in `HQL_NODES`, before you can then
    activate 'MIGRATE_DB_FROM' again. This will prevent you from overlaps and conflicts.
    After the migration has been done, you remove the 'MIGRATE_DB_FROM' and add more nodes again.
                "#
                );
            } else {
                warn!(
                    r#"

        Migrating data from 'MIGRATE_DB_FROM'
        This will overwrite possibly existing data in the current database!
        Make sure, that the 'MIGRATE_DB_FROM' was created with the same rauthy verion

        Proceeding in 10 seconds...

                "#
                );

                sleep(Duration::from_secs(10)).await;

                if from.starts_with("sqlite:") {
                    let pool_from = Self::connect_sqlite(&from, 1).await?;
                    if let Err(err) = db_migrate::migrate_from_sqlite(pool_from, &pool).await {
                        panic!("Error during db migration: {:?}", err);
                    }
                } else if from.starts_with("postgresql://") {
                    let pool_from = Self::connect_postgres(&from, 1).await?;
                    if let Err(err) = db_migrate::migrate_from_postgres(pool_from, &pool).await {
                        panic!("Error during db migration: {:?}", err);
                    }
                } else {
                    panic!(
                        "You provided an unknown database type, please check the MIGRATE_DB_FROM"
                    );
                };
            }
        } else if *DEV_MODE {
            migrate_dev_data(&pool).await.expect("Migrating DEV DATA");
        }

        if let Err(err) = db_migrate::anti_lockout(&pool, issuer).await {
            error!("Error when applying anti-lockout check: {:?}", err);
        }

        // update the DbVersion after successful pool creation and migrations
        DbVersion::upsert(&pool, db_version)
            .await
            .map_err(|err| anyhow::Error::msg(err.message))?;

        Ok(pool)
    }

    pub async fn connect_sqlite(
        addr: &str,
        max_conn: u32,
        // migration_only: bool,
    ) -> anyhow::Result<sqlx::SqlitePool> {
        let opts = sqlx::sqlite::SqliteConnectOptions::from_str(addr)?
            .create_if_missing(true)
            .busy_timeout(Duration::from_millis(100))
            .foreign_keys(true)
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = PoolOptions::new()
            .min_connections(2)
            .max_connections(max_conn)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(opts)
            .await
            .context("failed to connect to sqlite")?;

        info!("Database Connection Pool created successfully");

        Ok(pool)
    }

    pub async fn connect_postgres(addr: &str, max_conn: u32) -> anyhow::Result<sqlx::PgPool> {
        let opts = sqlx::postgres::PgConnectOptions::from_str(addr)?
            .log_slow_statements(LevelFilter::Debug, Duration::from_secs(3));
        let pool = PoolOptions::new()
            .min_connections(2)
            .max_connections(max_conn)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(opts)
            .await
            .context("failed to connect to postgres")?;

        info!("Database Connection Pool created successfully");

        Ok(pool)
    }
}

/// Helper to check if the current deployment is using multiple nodes
fn is_multi_replica_deployment() -> bool {
    NodeConfig::from_env().nodes.len() > 1
}

/// Holds the `argon2::Params` for the application.
///
/// This has been simplified a lot by now and it may be unwrapped and inserted into the
/// [AppState](AppState) directly later on. Needs some refactoring though.
#[derive(Debug, Clone)]
pub struct Argon2Params {
    pub params: argon2::Params,
}
