use crate::rauthy_config::{err_t, t_str, t_table};
use rauthy_common::constants::RAUTHY_VERSION;
use reqwest::{Client, tls};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::fs;
use toml::Value;
use tracing::debug;

#[derive(Default)]
pub struct VaultConfig {
    vault_source: VaultSource,
}

impl VaultConfig {
    pub async fn load_config() -> Result<String, Box<dyn Error>> {
        let vault_config_file = "vault.toml";
        let vault_conf = VaultConfig::build(vault_config_file).await?;

        let config_from_vault = vault_conf.vault_source.get_config().await?;
        let config = config_from_vault[&vault_conf.vault_source.config_key]
            .as_str()
            .expect("Failed to extract config from secrets.");

        Ok(config.to_string())
    }

    async fn build(config_file: &str) -> Result<Self, Box<dyn Error>> {
        let mut vault_conf = VaultConfig::default();

        if let Ok(cfg) = Self::load(config_file).await {
            vault_conf = cfg;
        }

        Ok(Self::load_from_env(vault_conf).await)
    }

    async fn load(path_vault_config: &str) -> Result<Self, Box<dyn Error>> {
        let config = fs::read_to_string(path_vault_config).await?;
        let vault_config = Self::load_from_string(&config).await?;
        Ok(vault_config)
    }

    async fn load_from_env(mut existing_config: VaultConfig) -> Self {
        if let Ok(v) = env::var("VAULT_ADDR") {
            existing_config.vault_source.addr = v;
        }
        if let Ok(v) = env::var("VAULT_TOKEN") {
            existing_config.vault_source.token = v;
        }
        if let Ok(v) = env::var("VAULT_MOUNT") {
            existing_config.vault_source.mount = v;
        }
        if let Ok(v) = env::var("VAULT_PATH") {
            existing_config.vault_source.path = v;
        }
        if let Ok(v) = env::var("VAULT_CONFIG_KEY") {
            existing_config.vault_source.config_key = v;
        }
        if let Ok(v) = env::var("VAULT_KV_VERSION") {
            match v.as_str() {
                "1" => existing_config.vault_source.kv_version = KvVersion::V1,
                "2" => existing_config.vault_source.kv_version = KvVersion::V2,
                _ => panic!("Invalid value for VAULT_KV_VERSION: {v}"),
            }
        }
        if let Ok(v) = env::var("HTTP_CUST_ROOT_CA_BUNDLE") {
            existing_config.vault_source.root_ca_bundle = Some(v);
        }

        existing_config
    }

    async fn load_from_string(config: &str) -> Result<Self, Box<dyn Error>> {
        let vault_source = VaultSource::default();
        let mut slf = Self { vault_source };

        let mut table = config
            .parse::<toml::Table>()
            .expect("Cannot parse TOML file");

        slf.parse_toml(&mut table);

        Ok(slf)
    }

    fn parse_toml(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "vault");

        self.vault_source.addr = table
            .remove("addr")
            .expect("missing in vault.toml: addr")
            .as_str()
            .unwrap()
            .to_owned();
        self.vault_source.token = table
            .remove("token")
            .expect("missing in vault.toml: token")
            .as_str()
            .unwrap()
            .to_owned();
        self.vault_source.mount = table
            .remove("mount")
            .expect("missing in vault.toml: mount")
            .as_str()
            .unwrap()
            .to_owned();
        self.vault_source.path = table
            .remove("path")
            .expect("missing in vault.toml: path")
            .as_str()
            .unwrap()
            .to_owned();
        self.vault_source.config_key = table
            .remove("config_key")
            .expect("missing in vault.toml: config_key")
            .as_str()
            .unwrap()
            .to_owned();

        if let Ok(v) = env::var("VAULT_KV_VERSION") {
            if v == "1" {
                self.vault_source.kv_version = KvVersion::V1;
            } else {
                self.vault_source.kv_version = KvVersion::V2;
            }
        } else {
            let Value::String(s) = table.remove("kv_version").expect("Missing kv_version") else {
                panic!("{}", err_t("kv_version", "vault", "String"));
            };
            if s == "1" {
                self.vault_source.kv_version = KvVersion::V1;
            } else {
                self.vault_source.kv_version = KvVersion::V2;
            }
        }

        if let Some(v) = t_str(
            &mut table,
            "vault",
            "root_ca_bundle",
            "HTTP_CUST_ROOT_CA_BUNDLE",
        ) {
            self.vault_source.root_ca_bundle = Some(v);
        }
    }
}

struct VaultSource {
    addr: String,
    token: String,
    mount: String,
    path: String,
    config_key: String,
    kv_version: KvVersion,
    root_ca_bundle: Option<String>,
}

impl Default for VaultSource {
    fn default() -> Self {
        VaultSource {
            addr: String::default(),
            token: String::default(),
            mount: String::default(),
            path: String::default(),
            config_key: String::default(),
            kv_version: KvVersion::V2,
            root_ca_bundle: None,
        }
    }
}

#[derive(PartialEq)]
enum KvVersion {
    V1 = 1,
    V2,
}

impl KvVersion {
    fn get_api_path(&self, mount: &str, path: &str) -> String {
        match self {
            KvVersion::V1 => format!("/v1/{mount}/{path}"),
            _ => format!("/v1/{mount}/data/{path}"),
        }
    }
}

impl VaultSource {
    fn build_kv_read_url(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_path = self.kv_version.get_api_path(&self.mount, path);
        let url = format!("{}{}", &self.addr, api_path);
        Ok(url)
    }
}

impl VaultSource {
    async fn get_config(
        &self,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        self.get_secrets(&self.path).await
    }

    async fn get_secrets(
        &self,
        path: &str,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let url = self.build_kv_read_url(path)?;
        let client = Self::http_client(self)?;
        let response = client
            .get(url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await?;

        if response.status().is_success() {
            let json = response.json::<serde_json::Value>().await?;
            let json_obj = json
                .get("data")
                .and_then(|x| {
                    if self.kv_version == KvVersion::V2 {
                        x.get("data")
                    } else {
                        Some(x)
                    }
                })
                .and_then(|x| x.as_object())
                .unwrap();

            let mut secret = HashMap::new();
            for (k, v) in json_obj {
                secret.insert(k.clone(), serde_json::Value::from(v.as_str().unwrap()));
            }

            return Ok(secret);
        }

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(Box::from(format!(
            "Failed to fetch secret from Vault (wrong kv version?). Status: {status} Body: {body}"
        )))
    }

    fn http_client(&self) -> Result<Client, Box<dyn std::error::Error>> {
        let insecure = env::var("DANGER_VAULT_INSECURE").as_deref() == Ok("true");

        let http_client = {
            let mut builder = reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(10))
                .min_tls_version(tls::Version::TLS_1_2)
                .user_agent(format!("Rauthy Client v{RAUTHY_VERSION}"))
                .https_only(!insecure)
                .danger_accept_invalid_certs(insecure)
                .use_rustls_tls();

            if let Some(bundle) = self.root_ca_bundle.clone() {
                let certs = reqwest::Certificate::from_pem_bundle(bundle.trim().as_bytes())
                    .expect("Cannot parse given HTTP_CUST_ROOT_CA_BUNDLE");
                debug!(
                    "Adding {} custom Root CA certificates to HTTP Client",
                    certs.len()
                );

                for cert in certs {
                    builder = builder.add_root_certificate(cert);
                }
            }

            builder.build()?
        };

        Ok(http_client)
    }
}
