use rauthy_client::device_code::DeviceCode;
use rauthy_client::{DangerAcceptInvalidCerts, RauthyHttpsOnly};
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut device_code = DeviceCode::request_with(
        "http://localhost:8080",
        "device".to_string(),
        Some("ZEW3dyKiPG27LdWrOFkNg6m9CaAEW5beoCxSufwdEBzSXMsxjNAXsnEbeI074d4V".to_string()),
        None,
        None,
        RauthyHttpsOnly::No,
        DangerAcceptInvalidCerts::Yes,
    )
    .await?;
    println!("{}", device_code);

    let _ts = device_code.wait_for_token().await?;

    // If the request has been verified, we have received a TokenSet at this point,
    // which we can use for furter requests.

    // TODO complete the example with qr code generation and fetch_userinfo()

    Ok(())
}
