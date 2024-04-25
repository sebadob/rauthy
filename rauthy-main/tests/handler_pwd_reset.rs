use crate::common::{check_status, get_backend_url, CLIENT_ID, CLIENT_SECRET};
use pretty_assertions::assert_eq;
use rauthy_common::constants::PWD_CSRF_HEADER;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::request::{PasswordResetRequest, TokenRequest};
use rauthy_service::token_set::TokenSet;
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;

mod common;

#[tokio::test]
async fn test_get_pwd_reset_form() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let user_id = "2PYV3STNz3MN7VnPjJVcPQap".to_string(); // test_admin
    let username = "test_admin@localhost.de";
    let reset_id = "2qqdUOcXECQeypBNTs7Pnp7A2zAwr0VzynyzJiIjNR1Ua9KA95dTewM56JaPIoyj";
    let url_get = format!("{backend_url}/users/{user_id}/reset/{reset_id}");

    let client = reqwest::Client::new();

    // try with bad magic link id
    println!("test_get_pwd_reset_form with bad magic link id");
    let bad_url_get = format!("{backend_url}/users/{user_id}/reset/iDoNotExist123");
    let res = client.get(&bad_url_get).send().await?;
    assert_eq!(res.status(), 404);

    // try with wrong user id
    println!("test_get_pwd_reset_form with wrong user id");
    let bad_url_get = format!("{backend_url}/users/2PYV3STNz3MN7VnPjJVcPQaX/reset/{reset_id}");
    let res = client.get(&bad_url_get).send().await?;
    assert_eq!(res.status(), 400);
    let err_html = res.text().await?;
    assert!(err_html.contains("request is malformed or incorrect"));

    // correct GET
    println!("test_get_pwd_reset_form correct GET");
    let mut res = client.get(&url_get).send().await?;
    res = check_status(res, 200).await?;

    // doing the same GET without the pwd_reset_cookie should fail now
    println!("test_get_pwd_reset_form without the pwd_reset_cookie");
    let res_2 = client.get(&url_get).send().await?;
    assert_eq!(res_2.status(), 403);
    let err_html = res_2.text().await?;
    assert!(
        err_html.contains("The requested password reset link is already tied to another session")
    );

    // get the password reset cookie
    let set_cookie = res.headers().get(header::SET_COOKIE).unwrap();
    let (cookie, _) = set_cookie.to_str()?.split_once(';').unwrap();
    println!("Extracted pwd_reset_cookie: {:?}", cookie);
    let mut correct_headers = HeaderMap::new();
    let mut cookie_only_headers = HeaderMap::new();
    correct_headers.append(header::COOKIE, HeaderValue::from_str(&cookie)?);
    cookie_only_headers.append(header::COOKIE, HeaderValue::from_str(&cookie)?);
    println!("Headers with cookie: {:?}", correct_headers);

    // check that we got the html document
    let html = res.text().await?;
    println!("{html}");
    assert!(html.starts_with("<!DOCTYPE html>"));

    // extract the csrf token from the html
    let (_, right) = html.split_once("name=\"rauthy-csrf-token\" id=\"").unwrap();
    let (csrf_token, _) = right.split_once("\"").unwrap();
    println!("Extracted csrf token: {csrf_token}");
    correct_headers.append(PWD_CSRF_HEADER, HeaderValue::from_str(csrf_token)?);
    println!("Headers with cookie + csrf: {:?}", correct_headers);

    let url_put = format!("{backend_url}/users/{user_id}/reset");

    // magic_link_id validation error
    let mut req = PasswordResetRequest {
        magic_link_id: "IAmSoWrong123".to_string(),
        password: "soSuperMegaSafe12345!\"§$%&/.".to_string(),
        mfa_code: None,
    };
    let mut res = client.put(&url_put).json(&req).send().await?;
    res = check_status(res, 400).await?;
    let err = res.text().await?;
    assert!(err.contains("magic_link_id"));

    // missing cookie - session already assigned
    println!("test_get_pwd_reset_form missing cookie - session already assigned");
    req.magic_link_id = reset_id.to_string();
    let res = client.put(&url_put).json(&req).send().await?;
    assert_eq!(res.status().as_u16(), 403);
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::Forbidden);
    assert_eq!(
        err.message,
        "The requested password reset link is already tied to another session"
    );

    // cookie only header -> csrf missing
    println!("test_get_pwd_reset_form cookie only header -> csrf missing");
    let res = client
        .put(&url_put)
        .json(&req)
        .headers(cookie_only_headers)
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 401);
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::Unauthorized);
    assert_eq!(err.message, "CSRF Token is missing");

    // correct headers - all should be good now
    let res = client
        .put(&url_put)
        .json(&req)
        .headers(correct_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 202);

    // now test logging in with the new password
    let url = format!("{}/oidc/token", get_backend_url());
    let body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        username: Some(username.to_string()),
        password: Some(req.password.to_string()),
        refresh_token: None,
    };
    let res = client.post(&url).form(&body).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    assert!(!ts.access_token.is_empty());
    assert!(ts.id_token.is_some());

    // test was used
    let res = client
        .put(&url_put)
        .json(&req)
        .headers(correct_headers)
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 400);
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert_eq!(err.message, "This link has expired already");

    Ok(())
}
