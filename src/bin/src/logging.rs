use std::env;
use tracing::Level;

// Sets up the logging / tracing depending on the env var `LOG_LEVEL`
pub fn setup_logging() -> tracing::Level {
    use tracing::Level;

    // setup logging
    dotenvy::dotenv().ok();
    let log_level = read_level("LOG_LEVEL");
    let log_level_db = read_level("LOG_LEVEL_DATABASE");

    let filter = format!(
        "{},cryptr=info,hyper=info,h2=info,openraft={}",
        log_level.as_str(),
        log_level_db.as_str(),
    );
    env::set_var("RUST_LOG", &filter);
    if log_level == Level::TRACE {
        env::set_var("RUST_BACKTRACE", "1");
    }

    if env::var("LOG_FMT").ok() == Some("json".to_string()) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .json()
            .with_max_level(log_level)
            .with_env_filter(filter)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    } else {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(log_level)
            .with_env_filter(filter)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    };

    log_level
}

fn read_level(env_var: &str) -> Level {
    match env::var(env_var)
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
    }
}
