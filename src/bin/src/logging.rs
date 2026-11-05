use rauthy_data::rauthy_config::RauthyConfig;
use tracing::Level;

// Sets up the logging / tracing depending on the env var `LOG_LEVEL`
pub fn setup_logging() -> tracing::Level {
    use tracing::Level;

    // setup logging
    let config = &RauthyConfig::get().vars.logging;
    let log_level = parse_level(&config.level);
    let log_level_db = parse_level(&config.level_database);

    let filter = format!(
        "{},cryptr=info,hyper=info,h2=info,hiqlite={},openraft={}",
        log_level.as_str(),
        log_level_db.as_str(),
        log_level_db.as_str(),
    );

    // Skip when RUST_BACKTRACE is set, the default hook already prints one then.
    if log_level == Level::TRACE && std::env::var_os("RUST_BACKTRACE").is_none() {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            default(info);
            eprintln!("{}", std::backtrace::Backtrace::force_capture());
        }));
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
    RauthyConfig::get().vars.logging.log_fmt.as_ref() == "json"
}

fn parse_level(level: &str) -> Level {
    match level {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => panic!("Log Level must be one of the following: error, warn, info, debug, trace"),
    }
}
