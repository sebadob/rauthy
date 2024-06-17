use crate::common::{get_auth_headers, get_backend_url};
use chrono::Utc;
use pretty_assertions::assert_eq;
use rauthy_models::request::IpBlacklistRequest;
use reqwest::StatusCode;
use std::error::Error;
use std::net::Ipv4Addr;
use std::ops::Add;
use std::time::Duration;

mod common;

#[tokio::test]
async fn test_ip_blacklist() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let auth_headers = get_auth_headers().await?;

    let url = format!("{}/blacklist", get_backend_url());
    // make sure we can GET blacklisted IPs
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // get the IP with which we are coming in at the backend from this test
    let url_ip = format!("{}/whoami", get_backend_url());
    let res = client.get(&url_ip).send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    let ip = res.text().await?.parse::<Ipv4Addr>().unwrap();
    println!("parsed ip: {:?}", ip);

    // let's blacklist ourselves
    // The blacklisting happens async and with second precision.
    // This means:
    // - blacklist for 2 seconds
    // - wait 1 second
    // - make sure we have no access
    // - wait 1 second
    // - we should have access again

    // blacklist for 2 seconds
    let exp = Utc::now().add(chrono::Duration::seconds(2)).timestamp();
    let payload = IpBlacklistRequest { ip, exp };
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // - wait 1 second
    tokio::time::sleep(Duration::from_secs(1)).await;

    // make sure we have no access
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
    let html = res.text().await?;
    assert!(html.contains("has been blocked"));
    assert!(html.contains("This incident has been reported"));

    // - wait 1 second
    tokio::time::sleep(Duration::from_secs(1)).await;

    // - we should have access again
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}
