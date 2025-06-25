use rauthy_models::rauthy_config::RauthyConfig;

/// Loads TLS key and cert file from disk and returns a `rustls::ServerConfig`
pub async fn load_tls() -> rustls::ServerConfig {
    let paths = &RauthyConfig::get().vars.tls;

    let key_path = paths
        .key_path
        .as_deref()
        .unwrap_or("tls/tls.key")
        .to_string();
    let cert_path = paths
        .cert_path
        .as_deref()
        .unwrap_or("tls/tls.crt")
        .to_string();

    tls_hot_reload::load_server_config(key_path, cert_path).await
}