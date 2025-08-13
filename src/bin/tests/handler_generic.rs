use crate::common::{get_backend_url, get_issuer};
use pretty_assertions::assert_eq;
use rauthy_data::entity::well_known::WellKnown;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_get_ping() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/ping", get_backend_url());
    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_get_well_known() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/.well-known/openid-configuration", get_backend_url());
    let res = reqwest::get(&url).await?;

    assert_eq!(res.status(), 200);
    let content = res.json::<WellKnown>().await?;
    assert_eq!(content.issuer, get_issuer());
    // don't test the rest for now as it might change soon again

    Ok(())
}
