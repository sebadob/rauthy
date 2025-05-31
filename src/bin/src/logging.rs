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

    // Unsafe because `env::set_var` has no locking under the hood,
    // which is not an issue in our case since this function is only called once at startup.
    unsafe {
        env::set_var("RUST_LOG", &filter);
    }

    if log_level == Level::TRACE {
        unsafe {
            env::set_var("RUST_BACKTRACE", "1");
        }
    }

    if is_log_fmt_json() {
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

pub fn is_log_fmt_json() -> bool {
    env::var("LOG_FMT")
        .map(|fmt| &fmt == "json")
        .unwrap_or(false)
}

fn read_level(env_var: &str) -> Level {
    match env::var(env_var)
        .as_deref()
        .unwrap_or("info")
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
