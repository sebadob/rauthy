use crate::rauthy_config::{err_t, t_str, t_table};
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::OnceLock;
use tokio::fs;
use toml::Value;
//use url::Url;

static CONFIG: OnceLock<VaultConfig> = OnceLock::new();
#[derive(Debug)]
pub struct VaultConfig {
    pub vault_source: VaultSource,
}

impl VaultConfig {
    pub fn new(vault_source: VaultSource) -> Self {
        Self { vault_source }
    }

    pub async fn load_vars() -> Result<String, Box<dyn Error>> {
        let vault_config_file = "vault.toml";
        let config_key = "config"; // maybe make configurable by env vars

        let vault_conf = VaultConfig::build(vault_config_file).await?;
        let config_from_vault = vault_conf.vault_source.get_config().await?;
        let config = config_from_vault[config_key].as_str().unwrap();

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
        Ok(Self::load_from_string(&config).await)
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
        if let Ok(v) = env::var("VAULT_PATH_CERTS") {
            existing_config.vault_source.path_certs = v;
        }
        if let Ok(v) = env::var("VAULT_KV_VERSION") {
            match v.as_str() {
                "1" => existing_config.vault_source.kv_version = KvVersion::V1,
                "2" => existing_config.vault_source.kv_version = KvVersion::V2,
                _ => panic!("Invalid value for VAULT_KV_VERSION: {}", v),
            }
        }

        existing_config
    }

    async fn load_from_string(config: &String) -> Self {
        let vault_source = VaultSource::default();
        let mut slf = Self::new(vault_source);

        let mut table = config
            .parse::<toml::Table>()
            .expect("Cannot parse TOML file");

        slf.parse_vault(&mut table);

        slf
    }

    fn parse_vault(&mut self, table: &mut toml::Table) {
        let Some(mut table) = t_table(table, "vault") else {
            return;
        };

        if let Some(v) = t_str(&mut table, "vault", "vault_addr", "VAULT_ADDR") {
            self.vault_source.addr = remove_quotes(&v);
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_token", "VAULT_TOKEN") {
            self.vault_source.token = remove_quotes(&v);
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_mount", "VAULT_MOUNT") {
            self.vault_source.mount = remove_quotes(&v);
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_path", "VAULT_PATH") {
            self.vault_source.path = remove_quotes(&v);
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_path_certs", "VAULT_PATH_CERTS") {
            self.vault_source.path_certs = remove_quotes(&v);
        }

        if let Ok(v) = env::var("KV_VERSION") {
            if v == "1" {
                self.vault_source.kv_version = KvVersion::V1;
            } else {
                self.vault_source.kv_version = KvVersion::V2;
            }
        } else {
            let Value::String(s) = table.remove("kv_version").unwrap() else {
                panic!("{}", err_t("kv_version", "vault", "String"));
            };
            if s == "1" {
                self.vault_source.kv_version = KvVersion::V1;
            } else {
                self.vault_source.kv_version = KvVersion::V2;
            }
        }
    }
}

impl Default for VaultConfig {
    fn default() -> Self {
        VaultConfig {
            vault_source: VaultSource::default(),
        }
    }
}
#[derive(Debug, Clone)]
struct VaultSource {
    addr: String,
    token: String,
    mount: String,
    path: String,
    path_certs: String,
    kv_version: KvVersion,
}

impl Default for VaultSource {
    fn default() -> Self {
        VaultSource {
            addr: "".to_string(),
            token: "".to_string(),
            mount: "".to_string(),
            path: "".to_string(),
            path_certs: "".to_string(),
            kv_version: KvVersion::V2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum KvVersion {
    V1 = 1,
    V2,
}

impl KvVersion {
    fn get_api_path(&self, mount: &str, path: &str) -> String {
        match self {
            KvVersion::V1 => format!("/v1/{}/{}", &mount, &path),
            _ => format!("/v1/{}/data/{}", &mount, &path),
        }
    }
}

// remove only the outer quotes
fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
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

    async fn get_certs(
        &self,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        self.get_secrets(&self.path_certs).await
    }

    async fn get_secrets(
        &self,
        path: &str,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let url = self.build_kv_read_url(path)?;
        let client = Client::new();
        let response = client
            .get(url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await?;

        if response.status().is_success() {
            let raw_text = &response.text().await.unwrap();
            let raw: serde_json::Value = serde_json::from_str(&raw_text)?;

            let json_obj = raw
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

        let status = response.error_for_status_ref().unwrap().status();
        let mut body = "".to_string();
        if let Ok(text) = &response.text().await {
            body = text.clone();
        }
        Err(Box::from(format!(
            "Failed to fetch secret from Vault (wrong kv version?). Status: {} Body: {}",
            status, body
        )))
    }
}
