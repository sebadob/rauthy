use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::{Language, PasswordPolicyRequest, PasswordPolicyResponse};
use rauthy_api_types::users::{NewUserRequest, UpdateUserRequest, UserResponse};
use rauthy_common::constants::PASSWORD_MAX_LEN_I32;
use rauthy_error::{ErrorResponse, ErrorResponseType};
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
    let rules = expect_status_json!(res, 200, PasswordPolicyResponse);
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
    let rules = expect_status_json!(res, 200, PasswordPolicyResponse);
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
    let rules = expect_status_json!(res, 200, PasswordPolicyResponse);
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
        given_name: Some("IT Test".to_string()),
        family_name: Some("Testy".to_string()),
        email: "alfred@batcave.io".to_string(),
        preferred_username: None,
        language: Language::En,
        roles: vec!["user".to_string()],
        groups: None,
        user_expires: None,
        tz: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_user)
        .send()
        .await?;
    let user = expect_status_json!(res, 200, UserResponse);

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
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("Minimum password length is 12"));

    upd_req.password = Some("sosuperdamnsafe".to_string());
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum upper character count: 3"));

    upd_req.password = Some("SOSUPERDAMNSAFE".to_string());
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum lower character count: 2"));

    upd_req.password = Some("soSuperDamnSafe".to_string());
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum digit count: 4"));

    upd_req.password = Some("1so2Super3Damn4Safe".to_string());
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("minimum special character count: 5"));

    upd_req.password = Some("!1so$%2Super/3D\"am\\n4^Safe'".to_string());
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let resp = expect_status_json!(res, 200, UserResponse);
    assert_eq!(resp.id, user.id);
    assert_eq!(resp.email, upd_req.email);
    assert_eq!(resp.enabled, true);

    // the password should not work again - recently used passwords constraint
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("last 7 used passwords"));

    // The policy's max length can be set to the hard request limit, which is needed to import a
    // user base from an application that allowed longer passwords than the previous 128 policy max.
    let new_rules = PasswordPolicyRequest {
        length_min: 12,
        length_max: PASSWORD_MAX_LEN_I32,
        include_lower_case: None,
        include_upper_case: None,
        include_digits: None,
        include_special: None,
        valid_days: Some(60),
        not_recently_used: Some(7),
    };
    let res = reqwest::Client::new()
        .put(&url)
        .headers(auth_headers.clone())
        .json(&new_rules)
        .send()
        .await?;
    let rules = expect_status_json!(res, 200, PasswordPolicyResponse);
    assert_eq!(rules.length_max, PASSWORD_MAX_LEN_I32);

    // A password beyond the old 256 chars request validator limit must be accepted by the API.
    upd_req.password = Some("aAbB1!2soSuperDamnSafe".repeat(15));
    let res = reqwest::Client::new()
        .put(&user_url)
        .headers(auth_headers.clone())
        .json(&upd_req)
        .send()
        .await?;
    let resp = expect_status_json!(res, 200, UserResponse);
    assert_eq!(resp.id, user.id);

    // Anything above the hard request limit stays rejected.
    let too_long = PasswordPolicyRequest {
        length_max: PASSWORD_MAX_LEN_I32 + 1,
        ..new_rules
    };
    let res = reqwest::Client::new()
        .put(&url)
        .headers(auth_headers.clone())
        .json(&too_long)
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("Payload validation error"));

    // Restore the strict rules for the tests that run after this one.
    let strict_rules = PasswordPolicyRequest {
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
        .json(&strict_rules)
        .send()
        .await?;
    let rules = expect_status_json!(res, 200, PasswordPolicyResponse);
    assert_eq!(rules.length_min, 12);
    assert_eq!(rules.length_max, 112);
    assert_eq!(rules.include_special, Some(5));

    Ok(())
}
