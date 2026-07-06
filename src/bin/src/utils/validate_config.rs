use crate::cli_args::ArgsValidateConfig;
use rauthy_data::email::mailer;
use rauthy_data::rauthy_config::RauthyConfig;
use std::error::Error;
use tokio::sync::mpsc;

pub async fn validate(args: ArgsValidateConfig) -> Result<(), Box<dyn Error>> {
    let (tx_email, _) = mpsc::channel::<mailer::EMail>(16);
    let (tx_events, _) = flume::unbounded();
    let (tx_events_router, _) = flume::unbounded();

    RauthyConfig::build(
        args.path.clone(),
        "secrets.toml".to_string(),
        tx_email,
        tx_events,
        tx_events_router,
    )
    .await?;

    println!("Rauthy Config '{}' is valid", args.path);

    Ok(())
}
