use std::sync::OnceLock;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tokio::fs;
use toml::Value;
use reqwest::Client;

static CONFIG: OnceLock<VaultConfig> = OnceLock::new();

#[derive(Debug)]
pub struct VaultConfig {
    pub vault_source: VaultSource,
}

impl VaultConfig {

    pub fn new(vault_source: VaultSource) -> Self {
        Self { vault_source }
    }

    pub async fn build(
        config_file: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let mut vault_conf = VaultConfig::default();

        match tokio::fs::try_exists(config_file).await {
            Ok(exists) => {
                if exists  {
                    vault_conf = Self::load(config_file).await;
                }
                else {
                    println!("{} not found.", config_file);
                }
            },
            Err(e) => {
                println!("Reading {} failed: {}", config_file, e);
            }
        }

        Ok(Self::load_from_env(vault_conf).await)
    }

    pub fn init_static(self) {
        CONFIG.set(self).unwrap();
    }

    #[inline(always)]
    pub fn get() -> &'static Self {
        CONFIG.get().unwrap()
    }

    async fn load(path_config: &str) -> Self {
        let Ok(config) = fs::read_to_string(path_config).await else {
            panic!("Cannot read config file from {}", path_config);
        };

        Self::load_from_string(&config).await
    }

    async fn load_from_env(mut existing_config: VaultConfig) -> Self {
        if let Ok(v) = env::var("VAULT_ADDR") {
            existing_config.vault_source.vault_addr = v;
        }
        if let Ok(v) = env::var("VAULT_TOKEN") {
            existing_config.vault_source.vault_token = v;
        }
        if let Ok(v) = env::var("VAULT_MOUNT") {
            existing_config.vault_source.vault_mount = v;
        }
        if let Ok(v) = env::var("VAULT_PATH") {
            existing_config.vault_source.vault_path = v;
        }
        if let Ok(v) = env::var("VAULT_PATH_CERTS") {
            existing_config.vault_source.vault_path_certs = v;
        }
        if let Ok(v) = env::var("VAULT_KV_VERSION") {
            match v.as_str() {
                "1" => existing_config.vault_source.kv_version = KvVersion::V1,
                _ => existing_config.vault_source.kv_version = KvVersion::V2,
            }
        }

        existing_config
    }    

    async fn load_from_string(config: &String) -> Self {
        let vault_source = VaultSource::default();
        let mut slf = Self::new(vault_source);

        // Note: these inner parsers are very verbose, but they allow the upfront memory allocation
        // and memory fragmentation, after the quite big toml has been freed and the config stays
        // in static memory.
        // It will also be easier in the future to maybe add other config sources into the mix
        // and use something else as default very easily, like e.g. a config fetched from a Vault.

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
            self.vault_source.vault_addr = v;
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_token", "VAULT_TOKEN") {
            self.vault_source.vault_token = v;
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_mount", "VAULT_MOUNT") {
            self.vault_source.vault_mount = v;
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_path", "VAULT_PATH") {
            self.vault_source.vault_path = v;
        }
        if let Some(v) = t_str(&mut table, "vault", "vault_path_certs", "VAULT_PATH_CERTS") {
            self.vault_source.vault_path_certs = v;
        }
        if let Some(v) = t_kv_version(&mut table, "vault", "kv_version", "KV_VERSION") {
            self.vault_source.kv_version = v;
        }
    }
}

impl Default for VaultConfig {
    fn default() -> Self {
         VaultConfig{
            vault_source: VaultSource::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct VaultSource {
    vault_addr: String,
    vault_token: String,
    vault_mount: String,
    vault_path: String,
    vault_path_certs: String,
    kv_version: KvVersion,
}

impl Default for VaultSource {
    fn default() -> Self {
         VaultSource{
            vault_addr: "".to_string(),
            vault_token: "".to_string(),
            vault_mount: "".to_string(),
            vault_path: "".to_string(),
            vault_path_certs: "".to_string(),
            kv_version: KvVersion::V2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KvVersion {
    V1 = 1,
    V2,
}

impl KvVersion {
    fn get_api_path(&self, mount: &str, path: &str) -> String {
         match self {
              KvVersion::V1 => format!("/v1/{}/{}", remove_quotes(&mount), remove_quotes(&path)),
              _ => format!("/v1/{}/data/{}", remove_quotes(&mount), remove_quotes(&path))
         }
    }
}

fn t_kv_version(map: &mut toml::Table, parent: &str, key: &str, env_var: &str) -> Option<KvVersion> {
    if !env_var.is_empty() {
        if let Ok(v) = env::var(env_var) {
            if v == "1" {
                return Some(KvVersion::V1);
            }
            else {
                return Some(KvVersion::V2);
            }
        }
    }
    let Value::String(s) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "String"));
    };
    if s == "1" {
        return Some(KvVersion::V1);
    }
    else {
        return Some(KvVersion::V2);
    }
}

fn t_str(map: &mut toml::Table, parent: &str, key: &str, env_var: &str) -> Option<String> {
    if !env_var.is_empty() {
        if let Ok(v) = env::var(env_var) {
            return Some(v);
        }
    }
    let Value::String(s) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "String"));
    };
    Some(s)
}

fn t_table(map: &mut toml::Table, key: &str) -> Option<toml::Table> {
    let Value::Table(t) = map.remove(key)? else {
        panic!("Expected type `Table` for {}", key)
    };
    Some(t)
}

#[inline]
fn err_t(key: &str, parent: &str, typ: &str) -> String {
    let sep = if parent.is_empty() { "" } else { "." };
    format!("Expected type `{}` for {}{}{}", typ, parent, sep, key)
}


// remove only the outer quotes
fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}

impl VaultSource {
    /// Creates a new instance of `VaultSource`.
    ///
    /// # Parameters
    ///
    /// * `vault_addr` - Complete URL of the Vault server (e.g. "http://127.0.0.1:8200")
    /// * `vault_token` - Authentication token for Vault
    /// * `vault_mount` - Name of the KV engine mount (e.g. "secret")
    /// * `vault_path` - Path to the secret within the mount (e.g. "rauthy_config")
    /// * `vault_path_cert` - Path to the secret containg certificates within the mount (e.g. "rauthy_certs")
    /// * `kv_version` - KV Version
    ///
    /// # Example
    ///
    /// ```
    /// use config_vault::VaultSource;
    ///
    /// let source = VaultSource::new(
    ///     "http://127.0.0.1:8200".to_string(),
    ///     "hvs.EXAMPLE_TOKEN".to_string(),
    ///     "secret".to_string(),
    ///     "rauthy_config".to_string(),
    ///     "rauthy_certs".to_string(),
    ///     KvVersion::V2,
    /// );
    /// ```
    pub fn new(
        vault_addr: String,
        vault_token: String,
        vault_mount: String,
        vault_path: String,
        vault_path_certs: String,
        kv_version: KvVersion
    ) -> Self {
        Self {
            vault_addr,
            vault_token,
            vault_mount,
            vault_path,
            vault_path_certs,
            kv_version,
        }        
    }

    /// Changes the KvVersion
    ///
    /// This function takes the target KvVersion and replaces the existing one.
    ///
    pub fn set_kv_version(&mut self, kv_version: KvVersion)
    {
        self.kv_version = kv_version;
    }

    /// Changes the Path to certs
    ///
    /// This function takes the target path for vault_path_certs and replaces the existing one.
    ///
    pub fn set_certs_path(&mut self, path: String)
    {
        self.vault_path_certs = path;
    }    

    /// Builds the URL for Vault's KV1/KV2 engine read API.
    ///
    /// This function takes the base address of Vault and builds the complete URL
    /// to access the read API of the KV1 engine with the specified path.
    ///
    /// # Returns
    ///
    /// * `Result<Url, ConfigError>` - The constructed URL or an error if the address is invalid
    //fn build_kv_read_url(&self) -> Result<Url, Box<dyn std::error::Error>> {
    fn build_kv_read_url(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_path = self.kv_version.get_api_path(&self.vault_mount,path);
        let url = format!("{}{}",remove_quotes(&self.vault_addr),api_path);
        Ok(url)
    }

}

impl VaultSource {
    pub async fn get_config(&self) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>>  {
        self.get_secrets(&self.vault_path).await
    }

    pub async fn get_certs(&self) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>>  {
        self.get_secrets(&self.vault_path_certs).await
    }

    pub async fn get_secrets(&self, path: &str) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>>  {
        let url = self.build_kv_read_url(path)?;
        let client = Client::new();
        let response = client
            .get(url)
            .header("X-Vault-Token", remove_quotes(&self.vault_token))
            .send().await?;

        if response.status().is_success() {

            let raw_text = &response.text().await.unwrap();        

            let raw: serde_json::Value = serde_json::from_str(&raw_text)?;

            let json_obj = raw
            .get("data")
            .and_then(|x| { if self.kv_version == KvVersion::V2 { x.get("data")} else { Some(x) } } )
            .and_then(|x| x.as_object())
            .unwrap();

            let mut secret = HashMap::new();
            for (k, v) in json_obj {
                secret.insert(k.clone(), serde_json::Value::from(v.as_str().unwrap()));
            }

            return Ok(secret)
        }
        
        Err(Box::from(format!("Failed to fetch secret from Vault (wrong kv version?): {}", response.status())))
    }
}