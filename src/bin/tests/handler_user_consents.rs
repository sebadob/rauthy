use crate::common::{
    CLIENT_ID, PASSWORD, USERNAME, decode_claims, get_auth_headers, get_backend_url,
    password_token_set, refresh, session_headers_with,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::users::{
    NewUserRequest, UpdateUserRequest, UserConsentDeleteResponse, UserConsentResponse,
    UserResponse, UserResponseSimple,
};
use rauthy_service::token_set::TokenSet;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::{Value, json};
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time;

mod common;

const EMAIL: &str = "consent-user@consenttest.io";
// satisfies the stricter policy of `zzb_handler_password_policy` as well
const PWD: &str = "Consent-TEST.1234!#$";

/// The refresh of a revoked refresh token fails on the missing row.
async fn assert_refresh_not_found(client: &Client, refresh_token: String) {
    let res = refresh(client, refresh_token).await;
    assert_eq!(res.status(), 404);
    let body = res.text().await.unwrap();
    assert!(body.contains("Refresh Token not found"), "{body}");
}

/// The password grant has no consent step, its refresh tokens are exempt from the consent check.
fn assert_consent_exempt(ts: &TokenSet) {
    let claims = decode_claims(ts.refresh_token.as_deref().unwrap());
    assert_eq!(claims["consent_exempt"], json!(true), "{claims}");
}

#[tokio::test]
async fn test_user_consents() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();

    let res = client
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?;
    let users = res.json::<Vec<UserResponseSimple>>().await?;
    let admin_id = users
        .iter()
        .find(|u| u.email == USERNAME)
        .unwrap()
        .id
        .clone();
    // left-over from a prior run
    if let Some(user) = users.iter().find(|u| u.email == EMAIL) {
        let res = client
            .delete(format!("{backend}/users/{}", user.id))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    // --- setup: a user with a usable password
    let res = client
        .post(format!("{backend}/users"))
        .headers(admin.clone())
        .json(&NewUserRequest {
            given_name: Some("Consent".to_string()),
            family_name: None,
            email: EMAIL.to_string(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["user".to_string()],
            groups: None,
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user = res.json::<UserResponse>().await?;

    let res = client
        .put(format!("{backend}/users/{}", user.id))
        .headers(admin.clone())
        .json(&UpdateUserRequest {
            email: user.email.clone(),
            given_name: user.given_name.clone(),
            family_name: user.family_name.clone(),
            language: Some(Language::En),
            password: Some(PWD.to_string()),
            roles: user.roles.clone(),
            groups: user.groups.clone(),
            enabled: true,
            email_verified: true,
            user_expires: None,
            user_values: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let url = format!("{backend}/users/{}/consents", user.id);
    let url_delete = format!("{url}?client_id={CLIENT_ID}&resource=");

    // --- authorization
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 401);
    let res = client.delete(&url_delete).send().await?;
    assert_eq!(res.status(), 401);

    let user_headers = session_headers_with(EMAIL, PWD).await;
    let res = client
        .get(format!("{backend}/users/{admin_id}/consents"))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);
    let res = client
        .delete(format!(
            "{backend}/users/{admin_id}/consents?client_id={CLIENT_ID}&resource="
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // --- list, the user itself and an admin
    for headers in [&user_headers, &admin] {
        let res = client.get(&url).headers(headers.clone()).send().await?;
        assert_eq!(res.status(), 200);
        assert!(res.json::<Vec<UserConsentResponse>>().await?.is_empty());
    }
    let res = client
        .get(format!("{url}?resource=https%3A%2F%2Fapi.example.com"))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = client
        .get(format!(
            "{url}?resource=https%3A%2F%2Fapi.example.com%23frag"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // --- validation: both params are required, `resource` without a fragment
    for query in [
        format!("client_id={CLIENT_ID}"),
        "resource=".to_string(),
        format!("client_id={CLIENT_ID}&resource=https%3A%2F%2Fapi.example.com%23frag"),
        "client_id=bad%20id&resource=".to_string(),
    ] {
        let res = client
            .delete(format!("{url}?{query}"))
            .headers(user_headers.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 400, "{query}");
    }

    // --- revocation drops the refresh tokens of the user for the client
    let ts = password_token_set(&client, EMAIL, PWD).await;
    assert_consent_exempt(&ts);

    time::sleep(Duration::from_secs(2)).await;
    let res = refresh(&client, ts.refresh_token.unwrap()).await;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    // the rotation keeps the exemption
    assert_consent_exempt(&ts);

    // another user's refresh token for the same client
    let other = password_token_set(&client, USERNAME, PASSWORD).await;

    let userinfo = async |access_token: &str| {
        client
            .get(format!("{backend}/oidc/userinfo"))
            .header(AUTHORIZATION, format!("Bearer {access_token}"))
            .send()
            .await
            .unwrap()
            .status()
    };
    assert_eq!(userinfo(&ts.access_token).await, 200);

    let events_from = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let res = client
        .delete(&url_delete)
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    // the password grant does not ask for consent, there is no consent row to delete
    assert!(!deleted.consent_deleted);
    // the rotated predecessor may still be there with a lowered `exp`
    assert!(deleted.refresh_tokens_deleted >= 1);

    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    // the access token issued with the deleted refresh token is revoked as well
    assert_eq!(userinfo(&ts.access_token).await, 401);
    // the other user keeps theirs
    let res = refresh(&client, other.refresh_token.unwrap()).await;
    assert_eq!(res.status(), 200);

    // idempotent, for an admin as well
    let res = client
        .delete(&url_delete)
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    assert!(!deleted.consent_deleted);
    assert_eq!(deleted.refresh_tokens_deleted, 0);

    // one event for the revoke that deleted something, none for the no-op
    time::sleep(Duration::from_secs(1)).await;
    let res = client
        .post(format!("{backend}/events"))
        .headers(admin.clone())
        .json(&json!({
            "from": events_from,
            "level": "info",
            "typ": "ConsentRevoked",
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let events = res.json::<Vec<Value>>().await?;
    let revoked = events
        .iter()
        .filter(|e| e["text"].as_str().is_some_and(|t| t.contains(EMAIL)))
        .count();
    assert_eq!(revoked, 1, "{events:?}");

    // --- a revoke with a `resource` drops the refresh tokens for the client as a whole
    let ts = password_token_set(&client, EMAIL, PWD).await;
    let res = client
        .delete(format!(
            "{url}?client_id={CLIENT_ID}&resource=https%3A%2F%2Fapi.example.com"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    assert!(!deleted.consent_deleted);
    assert!(deleted.refresh_tokens_deleted >= 1);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;

    // --- an unknown user
    let res = client
        .delete(format!(
            "{backend}/users/UnknownUserId1234/consents?client_id={CLIENT_ID}&resource="
        ))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 404);

    // --- cleanup
    let res = client
        .delete(format!("{backend}/users/{}", user.id))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}
