use crate::entity::db_version::DbVersion;
use crate::migration::db_migrate_dev::migrate_dev_data;
use crate::migration::{anti_lockout, db_migrate, init_prod};
use crate::rauthy_config::RauthyConfig;
use futures_util::StreamExt;
use hiqlite::cache_idx::CacheIndex;
use hiqlite_macros::embed::*;
use rauthy_common::{is_hiqlite, is_postgres};
use rauthy_error::ErrorResponse;
use std::env;
use std::ops::DerefMut;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::pin;
use tokio::time::sleep;
use tokio_postgres::config::{LoadBalanceHosts, SslMode};
use tracing::{debug, error, info, warn};

pub type PgClient = deadpool_postgres::Object;

static HIQLITE_CLIENT: OnceLock<hiqlite::Client> = OnceLock::new();
static PG_POOL: OnceLock<deadpool_postgres::Pool> = OnceLock::new();

mod migrations_postgres {
    refinery::embed_migrations!("../../migrations/postgres");
}

#[derive(Embed)]
#[folder = "../../migrations/hiqlite"]
struct MigrationsHiqlite;

/// Cache Index for the `hiqlite` cache layer
///
/// CAUTION: DO NOT change the order when adding new entries to now have false-positive
/// during updates for already existing environments. Caches are not indexed via String / Name,
/// but via u32 internally.
#[derive(Debug, strum::EnumIter)]
pub enum Cache {
    Atproto,
    App,
    AuthCode,
    DeviceCode,
    AuthProviderCallback,
    ClientDynamic,
    ClientEphemeral,
    ClientSecret,
    DPoPNonce,
    JwksRemote,
    ThemeTs,
    Html,
    IpBlacklist,
    IpRateLimit,
    Session,
    PoW,
    User,
    Webauthn,
    PAM,
    ToS,
}

impl CacheIndex for Cache {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub struct DB;

impl DB {
    /// Builds the NodeConfig and starts the Hiqlite node
    pub async fn init(config: hiqlite::NodeConfig) -> Result<(), ErrorResponse> {
        if HIQLITE_CLIENT.get().is_some() {
            panic!("DB::init() must only be called once at startup");
        }

        // Note: At the time of writing, I have not included the option to not start the DB
        // layer when the feature is enabled, as this would mean many adoptions and additional
        // checks in the Hiqlite code.
        // This means, even if Postgres is configured, this will still create and start the Hiqlite
        // DB, but it simply won't do anything else than heartbeats between the nodes.
        let client = hiqlite::start_node_with_cache::<Cache>(config).await?;

        if is_postgres() {
            Self::init_connect_postgres().await?;
        } else {
            client.wait_until_healthy_db().await;
            let mut metrics = client.metrics_db().await?;
            while metrics.state.is_learner() {
                info!("Waiting to become a full Raft member");
                sleep(Duration::from_secs(1)).await;
                metrics = client.metrics_db().await?;
            }
        }

        let _ = HIQLITE_CLIENT.set(client);

        Ok(())
    }

    /// Returns the static handle to the Hiqlite client
    #[inline]
    pub fn hql() -> &'static hiqlite::Client {
        HIQLITE_CLIENT
            .get()
            .expect("cache::start_cache() must be called at startup")
    }

    /// Returns a client from the Postgress connection pool
    #[inline]
    pub async fn pg() -> Result<PgClient, ErrorResponse> {
        PG_POOL
            .get()
            .expect("Database not initialized")
            .get()
            .await
            .map_err(ErrorResponse::from)
    }

    #[inline]
    pub async fn pg_txn_append(
        txn: &deadpool_postgres::Transaction<'_>,
        stmt: &str,
        params: &[&(dyn postgres_types::ToSql + Sync)],
    ) -> Result<u64, ErrorResponse> {
        let st = txn.prepare(stmt).await?;
        let rows_affected = txn.execute(&st, params).await?;
        Ok(rows_affected)
    }

    pub async fn connect_postgres(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        db_name: &str,
        db_max_conn: u16,
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

        let tls_config = if RauthyConfig::get().vars.database.pg_tls_no_verify {
            rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(NoTlsVerifier {}))
                .with_no_client_auth()
        } else {
            let root_store = rustls::RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
            };
            rustls::ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth()
        };
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);

        let mgr = deadpool_postgres::Manager::new(config, tls);
        let pool: deadpool_postgres::Pool = deadpool_postgres::Pool::builder(mgr)
            .max_size(db_max_conn as usize)
            .create_timeout(Some(Duration::from_secs(10)))
            .recycle_timeout(Some(Duration::from_secs(3600)))
            .runtime(deadpool::Runtime::Tokio1)
            .build()?;

        // check that we have a stable connection
        let client = pool.get().await?;
        client.simple_query("SELECT 1").await?;

        info!("Database Connection Pool created successfully");

        Ok(pool)
    }

    async fn init_connect_postgres() -> Result<(), ErrorResponse> {
        let cfg = &RauthyConfig::get().vars.database;
        let host = cfg.pg_host.as_ref().expect("PG_HOST is not set");
        let user = cfg.pg_user.as_ref().expect("PG_USER is not set");
        let password = cfg.pg_password.as_ref().expect("PG_PASSWORD is not set");
        let db_name = cfg.pg_db_name.as_ref();

        let pool =
            Self::connect_postgres(host, cfg.pg_port, user, password, db_name, cfg.pg_max_conn)
                .await?;

        PG_POOL
            .set(pool)
            .expect("DB::init_postgres() must only be called once at startup");

        Ok(())
    }

    pub async fn migrate() -> Result<(), ErrorResponse> {
        // before we do any db migrations, we need to check the current DB version
        // for compatibility
        let db_version = DbVersion::check_app_version().await?;

        if is_hiqlite() {
            Self::hql().migrate::<MigrationsHiqlite>().await?;
        } else {
            debug!("Migrating data from ../../migrations/postgres");
            let mut client = DB::pg().await?;
            let report = migrations_postgres::migrations::runner()
                .run_async(client.deref_mut().deref_mut())
                .await
                .expect("Error applying Postgres database migrations");
            debug!(?report, "Database Migration Report");
        }

        // migrate dynamic DB data
        let config = RauthyConfig::get();
        if !config.vars.dev.dev_mode && config.is_primary_node {
            init_prod::migrate_init_prod().await?;
        }

        if let Ok(from) = env::var("MIGRATE_DB_FROM") {
            if config.is_ha_cluster {
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
                        panic!("Error during db migration: {err:?}");
                    }
                } else if from.starts_with("postgres") {
                    if let Err(err) = db_migrate::migrate_from_postgres().await {
                        panic!("Error during db migration: {err:?}");
                    }
                } else {
                    panic!(
                        "You provided an unknown database type, please check the MIGRATE_DB_FROM"
                    );
                };
            }
        } else if config.vars.dev.dev_mode && config.is_primary_node {
            migrate_dev_data().await.expect("Migrating DEV DATA");
        }

        if let Err(err) = anti_lockout::anti_lockout().await {
            error!(?err, "applying anti-lockout check");
        }

        // update the DbVersion after successful pool creation and migrations
        DbVersion::upsert(db_version).await?;

        Ok(())
    }
}

// Postgres helpers
impl DB {
    /// Helper function to reduce boilerplate when doing raw postgres streaming queries.
    /// This is needed to make postgres `query_raw()` work easily.
    #[inline]
    pub fn params_iter<'a>(
        s: &'a [&'a (dyn postgres_types::ToSql + Sync)],
    ) -> impl ExactSizeIterator<Item = &'a dyn postgres_types::ToSql> + 'a {
        s.iter().map(|s| *s as _)
    }

    #[inline]
    pub async fn pg_execute(
        stmt: &str,
        params: &[&(dyn postgres_types::ToSql + Sync)],
    ) -> Result<usize, ErrorResponse> {
        let cl = Self::pg().await?;
        let st = cl.prepare_cached(stmt).await?;
        let rows_affected = cl.execute(&st, params).await?;
        // cast to usize for a uniform interface with Hiqlite
        Ok(rows_affected as usize)
    }

    #[inline]
    pub async fn pg_query_one<T: From<tokio_postgres::Row>>(
        stmt: &str,
        params: &[&(dyn postgres_types::ToSql + Sync)],
    ) -> Result<T, ErrorResponse> {
        let cl = Self::pg().await?;
        let st = cl.prepare_cached(stmt).await?;
        let row = cl.query_one(&st, params).await?;
        Ok(T::from(row))
    }

    #[inline]
    pub async fn pg_query_one_row(
        stmt: &str,
        params: &[&(dyn postgres_types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, ErrorResponse> {
        let cl = Self::pg().await?;
        let st = cl.prepare_cached(stmt).await?;
        let row = cl.query_one(&st, params).await?;
        Ok(row)
    }

    #[inline]
    pub async fn pg_query_opt<T: From<tokio_postgres::Row>>(
        stmt: &str,
        params: &[&(dyn postgres_types::ToSql + Sync)],
    ) -> Result<Option<T>, ErrorResponse> {
        let cl = Self::pg().await?;
        let st = cl.prepare_cached(stmt).await?;
        let row = cl.query_opt(&st, params).await?;
        match row {
            None => Ok(None),
            Some(row) => Ok(Some(T::from(row))),
        }
    }

    /// If you can estimate how many rows would be returned from the given query, provide a
    /// proper `expected_rows_size_hint` to reserve memory in advance. This will provide a small
    /// performance boost.
    #[inline]
    pub async fn pg_query<'a, T: From<tokio_postgres::Row>>(
        stmt: &str,
        params: &'a [&'a (dyn postgres_types::ToSql + Sync)],
        expected_rows_size_hint: usize,
    ) -> Result<Vec<T>, ErrorResponse> {
        Self::pg_query_map_with(&Self::pg().await?, stmt, params, expected_rows_size_hint).await
    }

    #[inline]
    pub async fn pg_query_map_with<'a, T: From<tokio_postgres::Row>>(
        client: &PgClient,
        stmt: &str,
        params: &'a [&'a (dyn postgres_types::ToSql + Sync)],
        expected_rows_size_hint: usize,
    ) -> Result<Vec<T>, ErrorResponse> {
        let st = client.prepare_cached(stmt).await?;
        let s = client.query_raw(&st, Self::params_iter(params)).await?;
        pin!(s);

        let mut res: Vec<T> = Vec::with_capacity(expected_rows_size_hint);
        while let Some(row) = s.next().await {
            res.push(T::from(row?));
        }

        Ok(res)
    }

    /// If you can estimate how many rows would be returned from the given query, provide a
    /// proper `expected_rows_size_hint` to reserve memory in advance. This will provide a small
    /// performance boost.
    #[inline]
    pub async fn pg_query_rows<'a>(
        stmt: &str,
        params: &'a [&'a (dyn postgres_types::ToSql + Sync)],
        expected_rows_size_hint: usize,
    ) -> Result<Vec<tokio_postgres::Row>, ErrorResponse> {
        let cl = Self::pg().await?;
        Self::pg_query_rows_with(&cl, stmt, params, expected_rows_size_hint).await
    }

    #[inline]
    pub async fn pg_query_rows_with<'a>(
        client: &PgClient,
        stmt: &str,
        params: &'a [&'a (dyn postgres_types::ToSql + Sync)],
        expected_rows_size_hint: usize,
    ) -> Result<Vec<tokio_postgres::Row>, ErrorResponse> {
        let st = client.prepare_cached(stmt).await?;
        let s = client.query_raw(&st, Self::params_iter(params)).await?;
        pin!(s);

        let mut res: Vec<tokio_postgres::Row> = Vec::with_capacity(expected_rows_size_hint);
        while let Some(row) = s.next().await {
            res.push(row?);
        }

        Ok(res)
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
            rustls::SignatureScheme::ED448,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ML_DSA_44,
            rustls::SignatureScheme::ML_DSA_65,
            rustls::SignatureScheme::ML_DSA_87,
        ]
    }
}
