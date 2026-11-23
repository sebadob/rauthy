use chrono::{Duration, NaiveDate, Utc};
use rauthy_common::compression::compress_file_gzip;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_schedulers::sleep_schedule_next;
use std::env;
use std::fs::create_dir_all;
use std::mem;
use std::path::PathBuf;
use std::str::FromStr;
use std::time;
use tokio::fs;
use tracing::{Level, debug, info, warn};

// Sets up the logging / tracing depending on the env var `LOG_LEVEL`
pub fn setup_logging() -> tracing::Level {
    use tracing::Level;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

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

    if let Some(log_file) = &config.log_file {
        let Some(prefix) = log_file.file_name() else {
            panic!(
                "`logging.log_file` must be the path to a file: {}",
                log_file.display()
            );
        };
        let prefix = prefix.to_string_lossy().to_string();
        let dir = match log_file.parent() {
            None => PathBuf::from("."),
            Some(p) if p.as_os_str().is_empty() => PathBuf::from("."),
            Some(p) => p.to_path_buf(),
        };

        create_dir_all(&dir)
            .unwrap_or_else(|err| panic!("Cannot create log file dir {}: {err}", dir.display()));

        let appender = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix(prefix.as_str())
            .build(&dir)
            .unwrap_or_else(|err| {
                panic!(
                    "Cannot init log file appender {}/{}: {}",
                    dir.display(),
                    prefix,
                    err
                )
            });
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);
        // if we don't forget the guard, it will be dropped immediately and logs will not work
        mem::forget(guard);

        // The `EnvFilter` is built from the very same `log_level`, so it already is the global
        // maximum level for this layer.
        let layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false);

        if is_log_fmt_json() {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new(&filter))
                .with(layer.json())
                .init();
        } else {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new(&filter))
                .with(layer)
                .init();
        }

        if !config.log_file_keep_for.is_zero() {
            let schedule = cron::Schedule::from_str(&config.log_file_cleanup)
                .expect("`logging.log_file_cleanup` is validated at startup");
            tokio::spawn(log_file_cleanup_task(
                dir,
                prefix,
                schedule,
                config.log_file_keep_for,
                config.log_file_archive_after,
            ));
        }
    } else if is_log_fmt_json() {
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
    }

    log_level
}

pub fn is_log_fmt_json() -> bool {
    RauthyConfig::get().vars.logging.log_fmt.as_ref() == "json"
}

/// Deletes rolling app log files older than `keep_for` from `dir`, compresses the ones older
/// than `archive_after` to `.gz` first, and runs on every occurrence of `schedule`. Files with
/// any unexpected name are ignored.
async fn log_file_cleanup_task(
    dir: PathBuf,
    prefix: String,
    schedule: cron::Schedule,
    keep_for: time::Duration,
    archive_after: Option<time::Duration>,
) {
    let Ok(retention) = Duration::from_std(keep_for) else {
        warn!("`logging.log_file_keep_for` is too large, the log file cleanup stays disabled");
        return;
    };
    let archive_retention = match archive_after.map(Duration::from_std) {
        None => None,
        Some(Ok(d)) => Some(d),
        Some(Err(_)) => {
            warn!(
                "`logging.log_file_archive_after` is too large, log file compression stays \
                disabled"
            );
            None
        }
    };

    loop {
        sleep_schedule_next(&schedule).await;

        let date_today = Utc::now().date_naive();
        let threshold_delete = date_today - retention;
        // One extra day: without it, an archive threshold of 5 days would fire 1 second after
        // midnight and only ever leave 4 uncompressed files.
        let threshold_archive = archive_retention.map(|a| date_today - a - Duration::days(1));
        debug!(
            "Cleaning up log files in {} - delete before {}",
            dir.display(),
            threshold_delete
        );

        match fs::read_dir(&dir).await {
            Err(err) => warn!("Cannot cleanup log files in {}: {}", dir.display(), err),
            Ok(mut entries) => loop {
                let Ok(Some(entry)) = entries.next_entry().await else {
                    break;
                };

                let fname = entry.file_name();
                let Some(suffix) = fname
                    .to_str()
                    .and_then(|name| name.strip_prefix(&prefix))
                    .and_then(|rest| rest.strip_prefix("."))
                else {
                    continue;
                };
                let date_str = suffix.strip_suffix(".gz").unwrap_or(suffix);
                let is_compressed = date_str != suffix;
                let Ok(date) = date_str.parse::<NaiveDate>() else {
                    continue;
                };

                let path = entry.path();
                if date < threshold_delete {
                    info!("Cleaning up log file {}", path.display());
                    if let Err(err) = fs::remove_file(&path).await {
                        warn!("Cannot remove log file {}: {}", path.display(), err);
                    }
                } else if let Some(threshold) = threshold_archive
                    && date < threshold
                    && !is_compressed
                {
                    let dest = path.with_added_extension("gz");
                    match compress_file_gzip(&path, &dest).await {
                        Ok(()) => {
                            info!("Compressing log file {}", path.display());
                            if let Err(err) = fs::remove_file(&path).await {
                                warn!("Cannot remove log file {}: {}", path.display(), err);
                            }
                        }
                        Err(err) => warn!("Cannot compress log file {}: {}", path.display(), err),
                    }
                }
            },
        }
    }
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
