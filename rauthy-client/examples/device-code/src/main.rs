use rauthy_client::device_code::DeviceCode;
use rauthy_client::rauthy_error::RauthyError;
use rauthy_client::token_set::OidcTokenSet;
use rauthy_client::{DangerAcceptInvalidCerts, RauthyHttpsOnly};
use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), RauthyError> {
    // the rauthy-client emits some tracing output
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let client_id = "device".to_string();

    let mut device_code = DeviceCode::request_with(
        "http://localhost:8080",
        client_id.clone(),
        None,
        None,
        None,
        RauthyHttpsOnly::No,
        DangerAcceptInvalidCerts::Yes,
    )
    .await?;
    println!("{}", device_code);
    println!(
        r#"
    You will get a complete uri with the code included as well
    for displaying in a QR code or something like that:
    {}"#,
        device_code.verification_uri_complete.as_ref().unwrap()
    );

    // with the `qrcode` feature endabled, we can render the
    // verification_uri_complete into one
    let qr = device_code.qr_string()?;
    println!("\n{}", qr);

    // we can get a QR as SVG as well, but this example can't display it.
    // let qr = device_code.qr_svg()?;

    let ts = device_code.wait_for_token().await?;
    println!("\nTokenSet on accept:\n{:?}", ts);

    let claims = ts.id_claims()?.unwrap();
    println!("\nWe get the user claims as well:\n{:?}", claims);

    // If the client is configured to receive a refresh token, we can
    // spawn a refresh handler that does all the magic behind the scenes.
    // This will consume the current OidcTokenSet.
    ts.into_refresh_handler_with(
        client_id,
        None,
        None,
        RauthyHttpsOnly::No,
        DangerAcceptInvalidCerts::Yes,
    )
    .await?;
    // After spawning it, you can get the currently saved tokens with
    let _access_token = OidcTokenSet::access_token().await?;
    let _id_token = OidcTokenSet::id_token().await?;

    println!("Sleeping now to show token refresh.");
    tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;

    Ok(())
}
