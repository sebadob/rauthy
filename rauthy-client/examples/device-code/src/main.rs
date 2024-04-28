use rauthy_client::device_code::DeviceCode;
use rauthy_client::{DangerAcceptInvalidCerts, RauthyHttpsOnly};
use tracing::{info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let device_code = DeviceCode::request_with(
        "http://localhost:8080",
        "device",
        None,
        None,
        None,
        RauthyHttpsOnly::No,
        DangerAcceptInvalidCerts::Yes,
    )
    .await?;
    info!("{:?}", device_code);

    Ok(())
}
