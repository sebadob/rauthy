use crate::common::{get_auth_headers, get_backend_url};
use chrono::Utc;
use rauthy_api_types::blacklist::IpBlacklistRequest;
use reqwest::StatusCode;
use std::error::Error;
use std::net::IpAddr;
use std::ops::Add;
use std::time::Duration;

// The same helper module the `integration` target compiles, included by path instead of
// duplicated. `dead_code` is off here because this target only uses a part of it.
#[allow(dead_code, unused_imports)]
#[path = "integration/common.rs"]
mod common;

/// This test blacklists the IP the suite connects from for two seconds, which answers every
/// request from that source with a 429 while it runs, so it lives in a test target of its own:
/// Cargo runs test binaries one after another and cannot overlap that window with any other area.
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
    expect_status!(res, 200);

    // get the IP with which we are coming in at the backend from this test
    let url_ip = format!("{}/whoami", get_backend_url());
    let res = client.get(&url_ip).send().await?;
    let res = expect_status!(res, 200);

    let text = res.text().await.unwrap();
    let (ip, _) = text.split_once('\n').unwrap();
    let ip = ip.parse::<IpAddr>().unwrap();
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
    expect_status!(res, 200);

    // - wait 1 second
    tokio::time::sleep(Duration::from_secs(1)).await;

    // make sure we have no access
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let html = expect_status!(res, StatusCode::TOO_MANY_REQUESTS)
        .text()
        .await?;
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
    expect_status!(res, 200);

    Ok(())
}
