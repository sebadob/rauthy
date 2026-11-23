use crate::common::{
    USER_PASSWORD, create_login_user, delete_user, get_auth_headers, get_backend_url,
};
use crate::expect_status_json;
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::{Language, PasswordPolicyRequest, PasswordPolicyResponse};
use rauthy_api_types::users::{UpdateUserRequest, UserResponse};
use rauthy_common::constants::PASSWORD_MAX_LEN_I32;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::HeaderMap;
use std::error::Error;

/// The policy every other suite builds its test passwords against, and the one this test re-applies
/// at the end: the strict rules it sets while it runs must not leak into the suites after it.
const SEEDED_POLICY: PasswordPolicyRequest = PasswordPolicyRequest {
    length_min: 14,
    length_max: 128,
    include_lower_case: Some(1),
    include_upper_case: Some(1),
    include_digits: Some(1),
    include_special: None,
    valid_days: Some(180),
    not_recently_used: Some(3),
};

/// The rules this test runs against itself. Every password a test sets for a user of its own
/// satisfies them, which is what makes swapping the shared policy while this runs safe.
const STRICT_RULES: PasswordPolicyRequest = PasswordPolicyRequest {
    length_min: 12,
    length_max: 112,
    include_lower_case: Some(2),
    include_upper_case: Some(3),
    include_digits: Some(4),
    include_special: Some(5),
    valid_days: Some(60),
    not_recently_used: Some(7),
};

#[tokio::test]
async fn test_password_policy() -> Result<(), Box<dyn Error>> {
    let fx = Fixture::new().await?;
    let user = &fx.user;

    // Set the rules this test runs against itself. It does not depend on the state any other suite
    // left behind, and it restores the seeded rules at the end.
    let rules = fx.put_policy(&STRICT_RULES).await?;
    assert_eq!(rules.length_min, 12, "the strict policy is applied");
    assert_eq!(rules.length_max, 112, "the strict policy is applied");
    assert_eq!(
        rules.include_lower_case,
        Some(2),
        "the strict policy is applied"
    );
    assert_eq!(
        rules.include_upper_case,
        Some(3),
        "the strict policy is applied"
    );
    assert_eq!(
        rules.include_digits,
        Some(4),
        "the strict policy is applied"
    );
    assert_eq!(
        rules.include_special,
        Some(5),
        "the strict policy is applied"
    );
    assert_eq!(rules.valid_days, Some(60), "the strict policy is applied");
    assert_eq!(
        rules.not_recently_used,
        Some(7),
        "the strict policy is applied"
    );

    // get the rules again and make sure they are the applied version
    let rules = fx.get_policy().await?;
    assert_eq!(rules.length_min, 12);
    assert_eq!(rules.length_max, 112);
    assert_eq!(rules.include_special, Some(5));

    // Set a password and enable
    let mut upd_req = UpdateUserRequest {
        email: user.email.clone(),
        given_name: user.given_name.clone(),
        family_name: user.family_name.clone(),
        language: Some(Language::De),
        password: Some("sosafe".to_string()),
        roles: user.roles.clone(),
        groups: None,
        enabled: true,
        email_verified: false,
        user_expires: None,
        user_values: None,
    };

    fx.reject(&upd_req, "Minimum password length is 12").await?;

    upd_req.password = Some("sosuperdamnsafe".to_string());
    fx.reject(&upd_req, "minimum upper character count: 3")
        .await?;

    upd_req.password = Some("SOSUPERDAMNSAFE".to_string());
    fx.reject(&upd_req, "minimum lower character count: 2")
        .await?;

    upd_req.password = Some("soSuperDamnSafe".to_string());
    fx.reject(&upd_req, "minimum digit count: 4").await?;

    upd_req.password = Some("1so2Super3Damn4Safe".to_string());
    fx.reject(&upd_req, "minimum special character count: 5")
        .await?;

    upd_req.password = Some("!1so$%2Super/3D\"am\\n4^Safe'".to_string());
    let resp = fx.put_user(&upd_req).await?;
    assert_eq!(resp.id, user.id);
    assert_eq!(resp.email, upd_req.email);
    assert_eq!(resp.enabled, true);

    // the password should not work again - recently used passwords constraint
    fx.reject(&upd_req, "last 7 used passwords").await?;

    // The policy's max length can be set to the hard request limit, which is needed to import a
    // user base from an application that allowed longer passwords than the old 128 policy max.
    let rules = fx
        .put_policy(&PasswordPolicyRequest {
            length_max: PASSWORD_MAX_LEN_I32,
            ..STRICT_RULES
        })
        .await?;
    assert_eq!(rules.length_max, PASSWORD_MAX_LEN_I32);

    // A password beyond the old 256 chars request validator limit must be accepted by the API.
    upd_req.password = Some("aAbB1!2soSuperDamnSafe".repeat(15));
    let resp = fx.put_user(&upd_req).await?;
    assert_eq!(resp.id, user.id);

    // Anything above the hard request limit stays rejected.
    let res = fx
        .http
        .put(format!("{}/password_policy", get_backend_url()))
        .headers(fx.admin.clone())
        .json(&PasswordPolicyRequest {
            length_max: PASSWORD_MAX_LEN_I32 + 1,
            ..STRICT_RULES
        })
        .send()
        .await?;
    let err = expect_status_json!(res, 400, ErrorResponse);
    assert_eq!(err.error, ErrorResponseType::BadRequest);
    assert!(err.message.contains("Payload validation error"));

    // Restore the seeded rules so the suites that run after this one get them back.
    let rules = fx.put_policy(&SEEDED_POLICY).await?;
    assert_eq!(rules.length_min, 14);
    assert_eq!(rules.length_max, 128);
    assert_eq!(rules.include_lower_case, Some(1));
    assert_eq!(rules.include_upper_case, Some(1));
    assert_eq!(rules.include_digits, Some(1));
    assert_eq!(rules.include_special, None);
    assert_eq!(rules.valid_days, Some(180));
    assert_eq!(rules.not_recently_used, Some(3));

    fx.teardown().await
}

/// The fixture of this test: the global policy endpoint and a user of its own to set passwords on.
struct Fixture {
    http: reqwest::Client,
    admin: HeaderMap,
    user: UserResponse,
}

impl Fixture {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let http = reqwest::Client::new();
        let admin = get_auth_headers().await?;
        let email = format!("policy-{}@localhost", new_store_id().to_lowercase());
        let user = create_login_user(&http, &admin, &email, "IT Test", USER_PASSWORD).await?;
        Ok(Self { http, admin, user })
    }

    /// PUTs `/password_policy` and returns the stored policy.
    async fn put_policy(
        &self,
        req: &PasswordPolicyRequest,
    ) -> Result<PasswordPolicyResponse, Box<dyn Error>> {
        let res = self
            .http
            .put(format!("{}/password_policy", get_backend_url()))
            .headers(self.admin.clone())
            .json(req)
            .send()
            .await?;
        Ok(expect_status_json!(res, 200, PasswordPolicyResponse))
    }

    /// GETs `/password_policy`.
    async fn get_policy(&self) -> Result<PasswordPolicyResponse, Box<dyn Error>> {
        let res = self
            .http
            .get(format!("{}/password_policy", get_backend_url()))
            .headers(self.admin.clone())
            .send()
            .await?;
        Ok(expect_status_json!(res, 200, PasswordPolicyResponse))
    }

    /// PUTs this fixture's user and returns the stored user.
    async fn put_user(&self, req: &UpdateUserRequest) -> Result<UserResponse, Box<dyn Error>> {
        let res = self
            .http
            .put(format!("{}/users/{}", get_backend_url(), self.user.id))
            .headers(self.admin.clone())
            .json(req)
            .send()
            .await?;
        Ok(expect_status_json!(res, 200, UserResponse))
    }

    /// PUTs this fixture's user, expecting the 400 whose message contains `msg`.
    async fn reject(&self, req: &UpdateUserRequest, msg: &str) -> Result<(), Box<dyn Error>> {
        let res = self
            .http
            .put(format!("{}/users/{}", get_backend_url(), self.user.id))
            .headers(self.admin.clone())
            .json(req)
            .send()
            .await?;
        let err = expect_status_json!(res, 400, ErrorResponse);
        assert_eq!(err.error, ErrorResponseType::BadRequest);
        assert!(
            err.message.contains(msg),
            "unexpected body: {}",
            err.message
        );
        Ok(())
    }

    /// Removes this fixture's user again.
    async fn teardown(&self) -> Result<(), Box<dyn Error>> {
        delete_user(&self.admin, &self.user.email).await
    }
}
