use std::sync::OnceLock;
use std::collections::HashMap;

use reqwest::Client;
//use url::Url;

static CONFIG: OnceLock<VaultConfig> = OnceLock::new();

#[derive(Debug)]
pub struct VaultConfig {
    pub vault_rauthy: VaultSource,
}

impl VaultConfig {

    pub fn new(vault_rauthy: VaultSource) -> Self {
        Self { vault_rauthy }
    }

    pub fn init_static(self) {
        CONFIG.set(self).unwrap();
    }

    #[inline(always)]
    pub fn get() -> &'static Self {
        CONFIG.get().unwrap()
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
    /// * `vault_path` - Path to the secret within the mount (e.g. "dev")
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
    ///     "dev".to_string(),
    ///     "dev_certs".to_string(),
    /// );
    /// ```
    pub fn new(
        vault_addr: String,
        vault_token: String,
        vault_mount: String,
        vault_path: String,
        vault_path_certs: String,
    ) -> Self {
        Self {
            vault_addr,
            vault_token,
            vault_mount,
            vault_path,
            vault_path_certs,
            kv_version: KvVersion::V2,
        }        
    }

    /// Creates a new instance of `VaultSource` with kv_version V1
    ///
    /// # Parameters
    ///
    /// * `vault_addr` - Complete URL of the Vault server (e.g. "http://127.0.0.1:8200")
    /// * `vault_token` - Authentication token for Vault
    /// * `vault_mount` - Name of the KV engine mount (e.g. "secret")
    /// * `vault_path` - Path to the secret within the mount (e.g. "dev")
    ///
    /// # Example
    ///
    /// ```
    /// use config_vault::VaultSource;
    ///
    /// let source = VaultSource::new_v1(
    ///     "http://127.0.0.1:8200".to_string(),
    ///     "hvs.EXAMPLE_TOKEN".to_string(),
    ///     "secret".to_string(),
    ///     "dev".to_string(),
    ///     "dev_certs".to_string(),
    /// );
    /// ```
    pub fn new_v1( 
        vault_addr: String,
        vault_token: String,
        vault_mount: String,
        vault_path: String,
        vault_path_certs: String,
    ) -> Self {
         Self {
             vault_addr,
             vault_token,
             vault_mount,
             vault_path,
             vault_path_certs,
             kv_version: KvVersion::V1,
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

        /*
        let mut url = Url::parse(&self.vault_addr).unwrap();
            //.map_err(|e| Err(Box::from(format!("Invalid Vault address URL: {}", e))));

        url.path_segments_mut()
            //.map_err(|_| Err(Box::from("Vault address URL cannot be a base".into())))
            .unwrap()
            .pop_if_empty() // Remove trailing slash if any
            .extend(api_path.split('/')); // Add the API path segments
        */

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
        println!("####### [VaultSource] get_secrets &self: {:?}", &self);
        let url = self.build_kv_read_url(path)?;


        let client = Client::new();
        let response = client
            .get(url)
            .header("X-Vault-Token", remove_quotes(&self.vault_token))
            //.send().await.unwrap();
            .send().await?;
            //.map_err(|e| Err(Box::new(e))).await.unwrap();

        println!("response: {:?}",response);

        if response.status().is_success() {

            let raw_text = &response.text().await.unwrap();        
            println!("raw_text: {:?}",&raw_text); 

            //let raw = &response
            //let raw: serde_json::Value = serde_json::from_str(&raw_text).unwrap();
            let raw: serde_json::Value = serde_json::from_str(&raw_text)?;
            //.map_err(|e| Err(Box::new(e))).await.unwrap();
            //.map_err(|e| Err(Box::new(e))).unwrap();
            println!("raw: {:?}",&raw);


            //let raw_text = &response.text().await.unwrap();        
            //println!("raw_text: {:?}",&raw_text);    

            let json_obj = raw
            .get("data")
            .and_then(|x| { if self.kv_version == KvVersion::V2 { x.get("data")} else { Some(x) } } )
            .and_then(|x| x.as_object())
            .unwrap();

            println!("json_obj: {:?}",&json_obj);

            let mut secret = HashMap::new();
            for (k, v) in json_obj {
                println!("k: {:?}",&k);
                println!("v: {:?}",&v);
                secret.insert(k.clone(), serde_json::Value::from(v.as_str().unwrap()));
            }
            
            /*
            let vault_secret_key = "config";

            let config_value = secret[vault_secret_key].clone();
            println!("config_value: {:?}",&config_value);

            //return Ok(config_value.as_str().unwrap().to_string());
            */

            return Ok(secret)
        }
        
        Err(Box::from(format!("Failed to fetch secret from Vault (wrong kv version?): {}", response.status())))
    }
   
}
