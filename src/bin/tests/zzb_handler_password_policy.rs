use crate::common::{check_status, get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::language::Language;
use rauthy_models::request::{NewUserRequest, PasswordPolicyRequest, UpdateUserRequest};
use rauthy_models::response::{PasswordPolicyResponse, UserResponse};
use std::error::Error;

mod common;

#[tokio::test]
async fn test_password_policy() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let url = format!("{}/password_policy", backend_url);

    // get current rules
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let rules = res.json::<PasswordPolicyResponse>().await?;
    assert_eq!(rules.length_min, 14);
    assert_eq!(rules.length_max, 128);
    assert_eq!(rules.include_lower_case, Some(1));
    assert_eq!(rules.include_upper_case, Some(1));
    assert_eq!(rules.include_digits, Some(1));
    assert_eq!(rules.include_special, None);
    assert_eq!(rules.valid_days, Some(180));
    assert_eq!(rules.not_recently_used, Some(3));

    // modify the rules
    let new_rules = PasswordPolicyRequest {
        length_min: 12,
        length_max: 112,
        include_lower_case: Some(2),
        include_upper_case: Some(3),
        include_digits: Some(4),
        include_special: Some(5),
        valid_days: Some(60),
        not_recently_used: Some(7),
    };
    let res = reqwest::Client::new()
        .put(&url)
        .headers(auth_headers.clone())
        .json(&new_rules)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let rules = res.json::<PasswordPolicyResponse>().await?;
    assert_eq!(rules.length_min, 12);
    assert_eq!(rules.length_max, 112);
    assert_eq!(rules.include_lower_case, Some(2));
    assert_eq!(rules.include_upper_case, Some(3));
    assert_eq!(rules.include_digits, Some(4));
    assert_eq!(rules.include_special, Some(5));
    assert_eq!(rules.valid_days, Some(60));
    assert_eq!(rules.not_recently_used, Some(7));

    // get rules again and make sure they are the updated version
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let rules = res.json::<PasswordPolicyResponse>().await?;
    assert_eq!(rules.length_min, 12);
    assert_eq!(rules.length_max, 112);
    assert_eq!(rules.include_lower_case, Some(2));
    assert_eq!(rules.include_upper_case, Some(3));
    assert_eq!(rules.include_digits, Some(4));
    assert_eq!(rules.include_special, Some(5));
    assert_eq!(rules.valid_days, Some(60));
    assert_eq!(rules.not_recently_used, Some(7));

    // get all users and check the admin user
    let url = format!("{}/users", get_backend_url());

    // post a new user
    let new_user = NewUserRequest {
        given_name: "IT Test".to_string(),
        family_name: "Testy".to_string(),
        email: "alfred@batcave.io".to_string(),
        language: Language::En,
        roles: vec!["user".to_string()],
        groups: None,
        user_expires: None,
    };
    let mut res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_user)
        .send()
        .await?;
    res = check_status(res, 200).await?;
    let user = res.json::<UserResponse>().await?;

    // Set a password and enable
    let mut upd_req = UpdateUserRequest {
        email: user.email,
        given_name: user.given_name,
        family_name: user.family_name,
        language: Some(Language::De),
        password: Some("sosafe".to_string()),
        roles: user.roles,
        groups: None,
        enabled: true,
        email_verified: false,
        user_expires: None,
        user_values: None,
    };
    let user_url = format!("{}/{}", url, user.id);
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 400).await?;
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("Minimum password length is 12"));

    upd_req.password = Some("sosuperdamnsafe".to_string());
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 400).await?;
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum upper character count: 3"));

    upd_req.password = Some("SOSUPERDAMNSAFE".to_string());
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 400).await?;
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum lower character count: 2"));

    upd_req.password = Some("soSuperDamnSafe".to_string());
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 400).await?;
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum digit count: 4"));

    upd_req.password = Some("1so2Super3Damn4Safe".to_string());
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 400).await?;
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum special character count: 5"));

    upd_req.password = Some("!1so$%2Super/3D\"am\\n4^Safe'".to_string());
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 200).await?;
    let resp = res.json::<UserResponse>().await?;
    assert_eq!(resp.id, user.id);
    assert_eq!(resp.email, upd_req.email);
    assert_eq!(resp.enabled, true);

    // the password should not work again - recently used passwords constraint
    let mut res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    res = check_status(res, 400).await?;
    let err = res.json::<ErrorResponse>().await?;
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("last 7 used passwords"));

    Ok(())
}
