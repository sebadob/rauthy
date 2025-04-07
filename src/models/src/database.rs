use crate::app_state::AppState;
use crate::entity::db_version::DbVersion;
use crate::migration::db_migrate_dev::migrate_dev_data;
use crate::migration::{anti_lockout, db_migrate, init_prod};
use actix_web::web;
use hiqlite::NodeConfig;
use rauthy_common::constants::{DATABASE_URL, DEV_MODE};
use rauthy_common::{is_hiqlite, is_postgres};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolOptions;
use sqlx::{ConnectOptions, Postgres};
use std::env;
use std::ops::DerefMut;
use std::str::FromStr;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::config::{LoadBalanceHosts, SslMode};
use tracing::log::LevelFilter;
use tracing::{debug, error, info, warn};

static CLIENT: OnceLock<hiqlite::Client> = OnceLock::new();
static PG_POOL_SQLX: OnceLock<sqlx::PgPool> = OnceLock::new();
static PG_POOL_DEADPOOL: OnceLock<deadpool_postgres::Pool> = OnceLock::new();

mod migrations_postgres {
    use refinery::embed_migrations;
    embed_migrations!("../../migrations/refinery");
}

#[derive(rust_embed::Embed)]
#[folder = "../../migrations/hiqlite"]
struct Migrations;

/// Cache Index for the `hiqlite` cache layer
#[derive(Debug, Serialize, Deserialize, hiqlite::EnumIter, hiqlite::ToPrimitive)]
pub enum Cache {
    App,
    AuthCode,
    DeviceCode,
    AuthProviderCallback,
    ClientDynamic,
    ClientEphemeral,
    DPoPNonce,
    JwksRemote,
    ThemeTs,
    Html,
    IPRateLimit,
    Session,
    PoW,
    User,
    Webauthn,
}

pub struct DB;

impl DB {
    /// Builds the NodeConfig and starts the Hiqlite node
    pub async fn init() -> Result<(), ErrorResponse> {
        if CLIENT.get().is_some() {
            panic!("DB::init() must only be called once at startup");
        }

        let config = NodeConfig::from_env();
        // Note: At the time of writing, I have not included the option to not start the DB
        // layer when the feature is enabled, as this would mean many adoptions and additional
        // checks in the Hiqlite code.
        // This means, even if Postgres is configured, this will still create and start the Hiqlite
        // DB, but it simply won't do anything else than heartbeats between the nodes.
        let client = hiqlite::start_node_with_cache::<Cache>(config).await?;
        let _ = CLIENT.set(client);

        if is_postgres() {
            Self::init_connect_postgres().await?;
        }

        Ok(())
    }

    /// Returns the static handle to the Hiqlite client
    pub fn client() -> &'static hiqlite::Client {
        CLIENT
            .get()
            .expect("cache::start_cache() must be called at startup")
    }

    /// Returns a Postgres connection.
    ///
    /// # Panics
    /// On logic errors in the code, as this must never be called with no configured Postgres DB.
    pub fn conn<'a>() -> &'a sqlx::PgPool {
        PG_POOL_SQLX
            .get()
            .expect("DB::conn() must never be called with no configured Postgres DB")
    }

    /// Returns a Postgres transaction.
    ///
    /// # Panics
    /// On logic errors in the code, as this must never be called with no configured Postgres DB.
    pub async fn txn<'a>() -> Result<sqlx::Transaction<'a, Postgres>, ErrorResponse> {
        let txn = PG_POOL_SQLX
            .get()
            .expect("DB::txn() must never be called with no configured Postgres DB")
            .begin()
            .await?;
        Ok(txn)
    }

    async fn connect_postgres_sqlx(
        url: &str,
        db_max_conn: u32,
    ) -> Result<sqlx::PgPool, ErrorResponse> {
        info!("Trying to connect to Postgres instance");
        let opts = sqlx::postgres::PgConnectOptions::from_str(url)?
            .log_slow_statements(LevelFilter::Debug, Duration::from_secs(3));
        let pool = PoolOptions::new()
            .min_connections(2)
            .max_connections(db_max_conn)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(opts)
            .await?;
        Ok(pool)
    }

    async fn connect_postgres(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        db_name: &str,
        db_max_conn: u32,
    ) -> Result<deadpool_postgres::Pool, ErrorResponse> {
        // let mut config: tokio_postgres::Config = db_host.parse().expect("invalid database url");
        let mut config: tokio_postgres::Config = tokio_postgres::Config::default();
        config.host(host);
        config.port(port);
        config.user(user);
        config.password(password);
        config.dbname(db_name);
        config.application_name("Rauthy");
        config.connect_timeout(Duration::from_secs(10));
        config.keepalives_idle(Duration::from_secs(30));
        // config.keepalives_interval()
        config.load_balance_hosts(LoadBalanceHosts::Random);
        config.ssl_mode(SslMode::Prefer);

        let tls_config = if let Some("true") = env::var("DATABASE_TLS_NO_VERIFY").ok().as_deref() {
            rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(NoTlsVerifier {}))
                .with_no_client_auth()
        } else {
            rustls::ClientConfig::builder()
                .with_root_certificates(rustls::RootCertStore::empty())
                .with_no_client_auth()
        };
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);

        let mgr = deadpool_postgres::Manager::new(config, tls);
        let pool: deadpool_postgres::Pool = deadpool_postgres::Pool::builder(mgr)
            .max_size(db_max_conn as usize)
            .create_timeout(Some(Duration::from_secs(10)))
            .recycle_timeout(Some(Duration::from_secs(3600)))
            .runtime(deadpool::Runtime::Tokio1)
            // .post_create(Hook::async_fn(move |client, _| {
            //     Box::pin(async move {
            //         on_connect(client).await.unwrap();
            //         // pg_pool::Test::prepare_find_all(client).await.unwrap();
            //         // pg_pool::Test::prepare_find(client).await.unwrap();
            //         // pg_pool::Test::prepare_insert(client).await.unwrap();
            //         Ok(())
            //     })
            // }))
            .build()?;

        // check that we have a stable connection
        let client = pool.get().await?;
        client.simple_query("SELECT 1").await?;

        info!("Database Connection Pool created successfully");

        // // Note: Postgres 15 denies to create anything in the public schema for any user but postgres.
        // sqlx::migrate!()
        //     .run(&db)
        //     .await
        //     .expect("Error running DB migrations");

        // database migrations
        let mut conn = pool.get().await?;
        let client = conn.deref_mut().deref_mut();
        let report = migrations_postgres::migrations::runner()
            .run_async(client)
            .await
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("refinery postgres migrations error: {:?}", err),
                )
            })?;
        debug!("{:?}", report);

        Ok(pool)
    }

    async fn init_connect_postgres() -> Result<(), ErrorResponse> {
        let db_max_conn = env::var("DATABASE_MAX_CONN")
            .unwrap_or_else(|_| "20".to_string())
            .parse::<u32>()
            .expect("Error parsing DATABASE_MAX_CONN to u32");

        if let Some(db_url) = DATABASE_URL.as_ref() {
            // BEGIN:sqlx - TODO remove the sqlx section after migration is finished
            let pool = Self::connect_postgres_sqlx(db_url, db_max_conn).await?;
            info!("Postgres database connection pool created successfully");

            PG_POOL_SQLX
                .set(pool)
                .expect("DB::init_postgres() must only be called once at startup");
            // END: sqlx

            let host = env::var("DATABASE_HOST").expect("DATABASE_HOST is not set");
            let port = env::var("DATABASE_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse::<u16>()
                .expect("Cannot parse DATABASE_PORT to u16");
            let user = env::var("DATABASE_USER").expect("DATABASE_USER is not set");
            let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set");
            let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set");
            let pool = Self::connect_postgres(&host, port, &user, &password, &db_name, db_max_conn)
                .await?;

            PG_POOL_DEADPOOL
                .set(pool)
                .expect("DB::init_postgres() must only be called once at startup");
        } else {
            panic!("DATABASE_URL is not set");
        }

        Ok(())
    }

    pub async fn migrate(app_state: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        // before we do any db migrations, we need to check the current DB version
        // for compatibility
        let db_version = DbVersion::check_app_version().await?;

        if is_hiqlite() {
            Self::client().migrate::<Migrations>().await?;
        } else if is_postgres() {
            debug!("Migrating data from ../../migrations/postgres");
            sqlx::migrate!("../../migrations/postgres")
                .run(Self::conn())
                .await
                .map_err(|err| {
                    ErrorResponse::new(
                        ErrorResponseType::Database,
                        format!("Postgres migration error: {:?}", err),
                    )
                })?;
        } else {
            unreachable!();
        }

        // migrate dynamic DB data
        if !*DEV_MODE {
            init_prod::migrate_init_prod(app_state.argon2_params.clone(), &app_state.issuer)
                .await?;
        }

        if let Ok(from) = env::var("MIGRATE_DB_FROM") {
            if NodeConfig::from_env().nodes.len() > 1 {
                // TODO does this error make sense or might we be able to do it anyway?
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
                    if let Err(err) = db_migrate::migrate_from_sqlite(&from).await {
                        panic!("Error during db migration: {:?}", err);
                    }
                } else if from.starts_with("postgresql://") {
                    let pool = Self::connect_postgres_sqlx(&from, 1).await?;
                    if let Err(err) = db_migrate::migrate_from_postgres(pool).await {
                        panic!("Error during db migration: {:?}", err);
                    }
                } else {
                    panic!(
                        "You provided an unknown database type, please check the MIGRATE_DB_FROM"
                    );
                };
            }
        } else if *DEV_MODE {
            migrate_dev_data(&app_state.issuer)
                .await
                .expect("Migrating DEV DATA");
        }

        if let Err(err) = anti_lockout::anti_lockout(&app_state.issuer).await {
            error!("Error when applying anti-lockout check: {:?}", err);
        }

        // update the DbVersion after successful pool creation and migrations
        DbVersion::upsert(db_version).await?;

        Ok(())
    }
}

/// Be very careful when you use this verifier. It will make any TLS connection work but does NOT
/// VALIDATE any certificates. Use is discouraged and it should only be done for testing.
#[derive(Debug)]
struct NoTlsVerifier {}

impl rustls::client::danger::ServerCertVerifier for NoTlsVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
        ]
    }
}
