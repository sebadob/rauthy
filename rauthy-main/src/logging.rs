use std::env;

// Sets up the logging / tracing depending on the env var `LOG_LEVEL`
pub fn setup_logging() -> tracing::Level {
    use tracing::Level;

    // setup logging
    dotenvy::dotenv().ok();
    let log_level = match env::var("LOG_LEVEL")
        .unwrap_or_else(|_| String::from("info"))
        .trim()
        .to_lowercase()
        .as_str()
    {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => panic!("Log Level must be one of the following: error, warn, info, debug, trace"),
    };
    let filter = format!(
        "{},async_nats=info,hyper=info,matrix_sdk_crypto=error",
        log_level.as_str()
    );
    env::set_var("RUST_LOG", &filter);
    if log_level == Level::TRACE {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    log_level
}
