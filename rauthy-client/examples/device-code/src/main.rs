use rauthy_client::device_code::DeviceCode;
use rauthy_client::rauthy_error::RauthyError;
use rauthy_client::{DangerAcceptInvalidCerts, RauthyHttpsOnly};

#[tokio::main]
async fn main() -> Result<(), RauthyError> {
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
    println!(
        r#"
    You will get a complete uri with the code included as well
    for displaying in a QR code or something like that:
    {}"#,
        device_code.verification_uri_complete.as_ref().unwrap()
    );

    let ts = device_code.wait_for_token().await?;
    println!("\nTokenSet on accept:\n{:?}", ts);

    let claims = ts.id_claims()?.unwrap();
    println!("\nWe get the user claims as well:\n{:?}", claims);

    Ok(())
}
