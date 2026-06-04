use rauthy_api_types::oidc::JktClaim;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtCommonClaims<'a> {
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub iss: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<&'a str>,
    pub aud: Cow<'a, str>,
    pub sub: Option<&'a str>,
    // pub nonce: Option<&'a str>,
    pub typ: JwtTokenType,
    pub azp: &'a str,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<&'a str>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim<'a>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddressClaim<'a> {
    pub formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<&'a str>,
    // pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
}

impl From<AddressClaim<'_>> for rauthy_api_types::oidc::AddressClaim {
    fn from(a: AddressClaim<'_>) -> Self {
        Self {
            formatted: a.formatted,
            street_address: a.street_address.map(String::from),
            locality: a.locality.map(String::from),
            postal_code: a.postal_code.map(String::from),
            country: a.country.map(String::from),
        }
    }
}

impl AddressClaim<'_> {
    pub fn try_build<'a>(user: &'a User, values: &'a UserValues) -> Option<AddressClaim<'a>> {
        let mut slf = AddressClaim {
            formatted: format!("{}\n", user.email_recipient_name()),
            street_address: None,
            locality: None,
            postal_code: None,
            country: None,
        };

        if let Some(street) = &values.street {
            writeln!(slf.formatted, "{street}").expect("AddressClaim to build");
            slf.street_address = Some(street);
        }

        if let Some(zip) = &values.zip {
            slf.postal_code = Some(zip);

            if let Some(city) = &values.city {
                writeln!(slf.formatted, "{zip}, {city}").expect("AddressClaim to build");
                slf.locality = Some(city);
            } else {
                writeln!(slf.formatted, "{zip}").expect("AddressClaim to build");
            }
        }

        if let Some(country) = &values.country {
            writeln!(slf.formatted, "{country}").expect("AddressClaim to build");
            slf.country = Some(country);
        }

        if slf.street_address.is_some()
            || slf.locality.is_some()
            || slf.postal_code.is_some()
            || slf.country.is_some()
        {
            Some(slf)
        } else {
            None
        }
    }
}

impl<'a> From<&'a rauthy_api_types::oidc::AddressClaim> for AddressClaim<'a> {
    fn from(value: &rauthy_api_types::oidc::AddressClaim) -> AddressClaim<'_> {
        AddressClaim {
            formatted: value.formatted.to_string(),
            street_address: value.street_address.as_deref(),
            locality: value.locality.as_deref(),
            postal_code: value.postal_code.as_deref(),
            country: value.country.as_deref(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtAccessClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    // pub scope: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<&'a str>>,
    // user part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
    /// Custom user attributes promoted to the token root, driven by the per-scope
    /// `claims_at_root` flag. Flattened, so each entry becomes a top-level claim
    /// instead of nesting under `custom`. Issuance MUST fail if any key here
    /// collides with a reserved claim; see [`validate_no_reserved_collision`].
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom_flattened: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtIdClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    pub amr: Vec<&'a str>,
    pub auth_time: i64,
    pub at_hash: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressClaim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_verified: Option<bool>,
    pub roles: Vec<String>, // TODO change to borrowed data when everything works
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>, // TODO change to borrowed data when everything works
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
    /// See [`JwtAccessClaims::custom_flattened`].
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom_flattened: Option<HashMap<String, serde_json::Value>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub webid: Option<Cow<'a, str>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub zoneinfo: Option<&'a str>,
}

/// JWT claim names that Rauthy may emit at the token root: the registered claims
/// from RFC 7519, the OIDC core / standard claims, and Rauthy's own top-level
/// fields on the access and id tokens.
///
/// A custom attribute promoted to the token root (per-scope `claims_at_root`) MUST
/// NOT reuse any of these names, otherwise the produced token would carry a
/// duplicate / shadowed key. This list is a first line of defense and is
/// intentionally **not** exhaustive (see issue #1595): the authoritative guarantee
/// is [`validate_no_reserved_collision`] failing issuance on a collision rather
/// than ever silently shadowing a claim.
pub const RESERVED_ROOT_CLAIMS: &[&str] = &[
    // RFC 7519 registered claims
    "iss",
    "sub",
    "aud",
    "exp",
    "nbf",
    "iat",
    "jti",
    // OIDC core (id token) claims
    "auth_time",
    "nonce",
    "acr",
    "amr",
    "azp",
    "at_hash",
    "c_hash",
    "sid",
    // OIDC standard claims
    "name",
    "given_name",
    "family_name",
    "middle_name",
    "nickname",
    "preferred_username",
    "profile",
    "picture",
    "website",
    "email",
    "email_verified",
    "gender",
    "birthdate",
    "zoneinfo",
    "locale",
    "phone_number",
    "phone_number_verified",
    "address",
    "updated_at",
    // Rauthy top-level fields (access / id tokens)
    "typ",
    "scope",
    "did",
    "cnf",
    "allowed_origins",
    "roles",
    "groups",
    "custom",
    "webid",
];

/// Ensures no key of a flattened, root-promoted custom claim map collides with a
/// [`RESERVED_ROOT_CLAIMS`] name. On collision, returns an error listing the
/// offending keys so the misconfigured scope can be fixed; the token is never
/// issued with a shadowed claim.
pub fn validate_no_reserved_collision(
    flattened: &HashMap<String, serde_json::Value>,
) -> Result<(), ErrorResponse> {
    let mut collisions = flattened
        .keys()
        .filter(|k| RESERVED_ROOT_CLAIMS.contains(&k.as_str()))
        .map(|k| k.as_str())
        .collect::<Vec<_>>();

    if collisions.is_empty() {
        return Ok(());
    }

    // sort for a stable, debuggable error message
    collisions.sort_unstable();
    Err(ErrorResponse::new(
        ErrorResponseType::Internal,
        format!(
            "Custom attribute(s) {collisions:?} are configured for root emission \
             (scope `claims_at_root`) but collide with reserved JWT claims. The \
             token was not issued to avoid shadowing a registered claim. Disable \
             `claims_at_root` for the affected scope or rename the attribute(s)."
        ),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtLogoutClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    pub events: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<&'a str>,

    // The `nonce` MUST NOT exist in this token. We try to deserialize into an `Option<_>` for easy
    // `.is_none()` validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtRefreshClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    pub uid: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum JwtTokenType {
    Bearer,
    DPoP,
    Id,
    #[serde(rename = "logout+jwt")]
    Logout,
    Refresh,
}

impl JwtTokenType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bearer => "Bearer",
            Self::DPoP => "DPoP",
            Self::Id => "Id",
            Self::Logout => "logout+jwt",
            Self::Refresh => "Refresh",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum JwtAmrValue {
    Pwd,
    Mfa,
}

impl FromStr for JwtAmrValue {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slf = match s {
            "pwd" => Self::Pwd,
            "mfa" => Self::Mfa,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Unknown value for 'amr' claim",
                ));
            }
        };
        Ok(slf)
    }
}

impl Display for JwtAmrValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl JwtAmrValue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pwd => "pwd",
            Self::Mfa => "mfa",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        JwtAccessClaims, JwtCommonClaims, JwtIdClaims, JwtTokenType, validate_no_reserved_collision,
    };
    use serde_json::json;
    use std::borrow::Cow;
    use std::collections::HashMap;

    fn common() -> JwtCommonClaims<'static> {
        JwtCommonClaims {
            iat: 1_700_000_000,
            nbf: 1_700_000_000,
            exp: 1_700_003_600,
            iss: "https://auth.example.com",
            jti: Some("jti123"),
            aud: Cow::Borrowed("client-id"),
            sub: Some("user-id"),
            typ: JwtTokenType::Bearer,
            azp: "client-id",
            scope: Some(Cow::Borrowed("openid profile oap")),
            did: None,
            cnf: None,
        }
    }

    // (c) Collision against a reserved claim must fail to issue, naming the offenders.
    #[test]
    fn collision_check_passes_clean_and_fails_on_reserved() {
        let mut clean = HashMap::new();
        clean.insert("oap_user_id".to_string(), json!("u1"));
        clean.insert("oap_org_id".to_string(), json!("o1"));
        assert!(validate_no_reserved_collision(&clean).is_ok());

        let mut bad = HashMap::new();
        bad.insert("email".to_string(), json!("x@y.z")); // reserved
        bad.insert("sub".to_string(), json!("spoofed")); // reserved
        bad.insert("oap_user_id".to_string(), json!("u1")); // fine
        let err = validate_no_reserved_collision(&bad).unwrap_err();
        assert!(err.message.contains("email"), "must name `email`");
        assert!(err.message.contains("sub"), "must name `sub`");
        assert!(
            !err.message.contains("oap_user_id"),
            "must not flag a non-reserved key"
        );
    }

    // (b) Flatten vs nested both serialize: flattened keys at root, nested under `custom`.
    #[test]
    fn access_token_flattened_at_root_nested_under_custom() {
        let mut flattened = HashMap::new();
        flattened.insert("oap_user_id".to_string(), json!("u-123"));
        flattened.insert("platform_role".to_string(), json!("admin"));
        let mut nested = HashMap::new();
        nested.insert("department".to_string(), json!("eng"));

        let claims = JwtAccessClaims {
            common: common(),
            allowed_origins: None,
            email: Some("a@b.c"),
            email_verified: Some(true),
            roles: None,
            groups: None,
            custom: Some(nested),
            custom_flattened: Some(flattened),
        };

        let v = serde_json::to_value(&claims).unwrap();
        // flattened entries become top-level claims
        assert_eq!(v["oap_user_id"], json!("u-123"));
        assert_eq!(v["platform_role"], json!("admin"));
        // nested entries stay under `custom`, never at root
        assert_eq!(v["custom"]["department"], json!("eng"));
        assert!(v.get("department").is_none());
        // reserved / common claims are unaffected
        assert_eq!(v["iss"], json!("https://auth.example.com"));
        assert_eq!(v["email"], json!("a@b.c"));
    }

    // (b) Round-trip via `from_slice` (mirrors token introspection in `token_info.rs`):
    // borrowed deserialization still works, and the greedy flatten map must NOT absorb
    // reserved / known root claims.
    #[test]
    fn access_token_round_trips_without_leaking_reserved() {
        let mut flattened = HashMap::new();
        flattened.insert("oap_user_id".to_string(), json!("u-123"));
        let mut nested = HashMap::new();
        nested.insert("department".to_string(), json!("eng"));

        let claims = JwtAccessClaims {
            common: common(),
            allowed_origins: None,
            email: Some("a@b.c"),
            email_verified: Some(true),
            roles: None,
            groups: None,
            custom: Some(nested),
            custom_flattened: Some(flattened),
        };

        let bytes = serde_json::to_vec(&claims).unwrap();
        let back = serde_json::from_slice::<JwtAccessClaims>(&bytes).unwrap();

        let cf = back.custom_flattened.expect("custom_flattened recovered");
        assert_eq!(cf.get("oap_user_id").unwrap(), &json!("u-123"));
        for reserved in [
            "iss",
            "sub",
            "aud",
            "exp",
            "nbf",
            "iat",
            "jti",
            "email",
            "email_verified",
            "custom",
            "scope",
            "azp",
            "typ",
        ] {
            assert!(
                !cf.contains_key(reserved),
                "custom_flattened leaked reserved claim `{reserved}`"
            );
        }
        let c = back.custom.expect("custom recovered");
        assert_eq!(c.get("department").unwrap(), &json!("eng"));
        // borrowed fields survived flatten-based deserialization
        assert_eq!(back.email, Some("a@b.c"));
        assert_eq!(back.common.iss, "https://auth.example.com");
    }

    // (b) Same guarantees for the id token.
    #[test]
    fn id_token_flattened_round_trips() {
        let mut flattened = HashMap::new();
        flattened.insert("oap_org_slug".to_string(), json!("acme"));
        let mut nested = HashMap::new();
        nested.insert("department".to_string(), json!("eng"));

        let claims = JwtIdClaims {
            common: common(),
            amr: vec!["pwd"],
            auth_time: 1_700_000_000,
            at_hash: "hash",
            sid: None,
            email: None,
            email_verified: None,
            preferred_username: None,
            given_name: None,
            family_name: None,
            address: None,
            birthdate: None,
            picture: None,
            locale: None,
            nonce: None,
            phone_number: None,
            phone_number_verified: None,
            roles: vec![],
            groups: None,
            custom: Some(nested),
            custom_flattened: Some(flattened),
            webid: None,
            zoneinfo: None,
        };

        let v = serde_json::to_value(&claims).unwrap();
        assert_eq!(v["oap_org_slug"], json!("acme"));
        assert_eq!(v["custom"]["department"], json!("eng"));

        let bytes = serde_json::to_vec(&claims).unwrap();
        let back = serde_json::from_slice::<JwtIdClaims>(&bytes).unwrap();
        let cf = back
            .custom_flattened
            .expect("id custom_flattened recovered");
        assert_eq!(cf.get("oap_org_slug").unwrap(), &json!("acme"));
        for reserved in ["amr", "auth_time", "at_hash", "custom", "iss", "exp"] {
            assert!(
                !cf.contains_key(reserved),
                "id custom_flattened leaked reserved claim `{reserved}`"
            );
        }
    }

    // (a) No custom value can produce shadowed JSON: a flattened map carrying a reserved
    // key is rejected before it can ever be serialized into a token.
    #[test]
    fn flattened_reserved_key_is_rejected_before_emit() {
        let mut flattened = HashMap::new();
        flattened.insert("groups".to_string(), json!(["forged-admin"]));
        assert!(validate_no_reserved_collision(&flattened).is_err());
    }
}
