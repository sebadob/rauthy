use crate::rauthy_config::{check_table_empty, t_str, t_str_vec, t_table};
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct RauthySecrets {
    pub cluster: toml::Table,
    pub database: VarsDatabase,
    pub dynamic_clients: VarsDynamicClients,
    pub email: VarsEmail,
    pub encryption: VarsEncryption,
    pub events: VarsEvents,
    pub geo: ConfigVarsGeo,
    pub user_pictures: VarsUserPictures,
}

#[derive(Default)]
pub struct VarsDatabase {
    pub pg_user: Option<String>,
    pub pg_password: Option<String>,
    pub migrate_pg_user: Option<String>,
    pub migrate_pg_password: Option<String>,
}

#[derive(Default)]
pub struct VarsDynamicClients {
    pub reg_token: Option<String>,
}

#[derive(Default)]
pub struct VarsEmail {
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub xoauth_client_id: Option<String>,
    pub xoauth_client_secret: Option<String>,
}

#[derive(Default)]
pub struct VarsEncryption {
    pub key_active: String,
    pub keys: Vec<String>,
}

#[derive(Default)]
pub struct VarsEvents {
    pub matrix_user_id: Option<String>,
    pub matrix_access_token: Option<String>,
    pub matrix_user_password: Option<String>,
    pub slack_webhook: Option<String>,
}

#[derive(Default)]
pub struct ConfigVarsGeo {
    pub maxmind_account_id: Option<String>,
    pub maxmind_license_key: Option<String>,
}

#[derive(Default)]
pub struct VarsUserPictures {
    pub s3_key: Option<String>,
    pub s3_secret: Option<String>,
}

impl RauthySecrets {
    pub fn read_or_default<P: AsRef<Path>>(path: P) -> Self {
        // The secrets file is optional, so we don't want to panic if it does not exist.
        let Ok(file) = fs::read_to_string(path) else {
            return Self::default();
        };

        let mut table = file
            .parse::<toml::Table>()
            .expect("Cannot parse Secrets TOML file");

        let mut slf = Self::default();

        slf.parse_database(&mut table);
        slf.parse_dynamic_clients(&mut table);
        slf.parse_email(&mut table);
        slf.parse_encryption(&mut table);
        slf.parse_events(&mut table);
        slf.parse_geo(&mut table);
        slf.parse_user_pictures(&mut table);

        slf.cluster = t_table(&mut table, "cluster");

        slf
    }

    fn parse_database(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "database");

        self.database.pg_user = t_str(&mut table, "database", "pg_user", "PG_USER");
        self.database.pg_password = t_str(&mut table, "database", "pg_password", "PG_PASSWORD");
        self.database.migrate_pg_user =
            t_str(&mut table, "database", "migrate_pg_user", "MIGRATE_PG_USER");
        self.database.migrate_pg_password = t_str(
            &mut table,
            "database",
            "migrate_pg_password",
            "MIGRATE_PG_PASSWORD",
        );

        check_table_empty(table, "secrets: database");
    }

    fn parse_dynamic_clients(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "dynamic_clients");

        self.dynamic_clients.reg_token = t_str(
            &mut table,
            "dynamic_clients",
            "reg_token",
            "DYN_CLIENT_REG_TOKEN",
        );

        check_table_empty(table, "secrets: dynamic_clients");
    }

    fn parse_email(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "email");

        self.email.smtp_username = t_str(&mut table, "email", "smtp_username", "SMTP_USERNAME");
        self.email.smtp_password = t_str(&mut table, "email", "smtp_password", "SMTP_PASSWORD");
        self.email.xoauth_client_id = t_str(
            &mut table,
            "email",
            "xoauth_client_id",
            "SMTP_XOAUTH2_CLIENT_ID",
        );
        self.email.xoauth_client_secret = t_str(
            &mut table,
            "email",
            "xoauth_client_secret",
            "SMTP_XOAUTH2_CLIENT_SECRET",
        );

        check_table_empty(table, "secrets: email");
    }

    fn parse_encryption(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "encryption");

        if let Some(v) = t_str(&mut table, "encryption", "key_active", "ENC_KEY_ACTIVE") {
            self.encryption.key_active = v;
        }
        if let Some(v) = t_str_vec(&mut table, "encryption", "keys", "ENC_KEYS") {
            self.encryption.keys = v;
        }

        check_table_empty(table, "secrets: encryption");
    }

    fn parse_events(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "events");

        self.events.matrix_user_id = t_str(
            &mut table,
            "events",
            "matrix_user_id",
            "EVENT_MATRIX_USER_ID",
        );
        self.events.matrix_access_token = t_str(
            &mut table,
            "events",
            "matrix_access_token",
            "EVENT_MATRIX_ACCESS_TOKEN",
        );
        self.events.matrix_user_password = t_str(
            &mut table,
            "events",
            "matrix_user_password",
            "EVENT_MATRIX_USER_PASSWORD",
        );
        self.events.slack_webhook =
            t_str(&mut table, "events", "slack_webhook", "EVENT_SLACK_WEBHOOK");

        check_table_empty(table, "secrets: events");
    }

    fn parse_geo(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "geolocation");

        self.geo.maxmind_account_id = t_str(
            &mut table,
            "geolocation",
            "maxmind_account_id",
            "GEO_MAXMIND_ACC_ID",
        );
        self.geo.maxmind_license_key = t_str(
            &mut table,
            "geolocation",
            "maxmind_license_key",
            "GEO_MAXMIND_LICENSE",
        );

        check_table_empty(table, "secrets: geolocation");
    }

    fn parse_user_pictures(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "user_pictures");

        self.user_pictures.s3_key = t_str(&mut table, "user_pictures", "s3_key", "PIC_S3_KEY");
        self.user_pictures.s3_secret =
            t_str(&mut table, "user_pictures", "s3_secret", "PIC_S3_SECRET");

        check_table_empty(table, "secrets: user_pictures");
    }
}
