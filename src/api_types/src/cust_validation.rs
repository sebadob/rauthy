use rauthy_common::constants::CLIENT_CLAIMS_MAX_LEN;
use rauthy_common::regex::{
    RE_ATTR, RE_CODE_CHALLENGE_METHOD, RE_CONTACT, RE_GRANT_TYPES, RE_GROUPS, RE_LINUX_HOSTNAME,
    RE_ORIGIN, RE_RESOURCE, RE_ROLES_SCOPES, RE_URI,
};
use std::borrow::Cow;
use validator::ValidationError;

#[inline]
pub fn validate_vec_attr(value: &[String]) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new(
            "'validate_vec_attr' cannot be empty when provided",
        ));
    }
    for v in value {
        if !RE_ATTR.is_match(v) {
            return Err(ValidationError::new("^[a-z0-9-_/]{2,128}$"));
        }
    }
    Ok(())
}

/// Custom claims attached to a client and emitted into `client_credentials`
/// tokens. Must be a JSON object (it is emitted nested under the `custom` claim)
/// and is capped to keep tokens small, since they are frequently stored in
/// cookies. The cap applies to the client's custom JSON only, not the full token.
#[inline]
pub fn validate_claims(value: &serde_json::Value) -> Result<(), ValidationError> {
    if !value.is_object() {
        return Err(ValidationError::new("`claims` must be a JSON object"));
    }

    let len = serde_json::to_vec(value)
        .map(|v| v.len())
        .unwrap_or(usize::MAX);
    if len > CLIENT_CLAIMS_MAX_LEN {
        return Err(
            ValidationError::new("claims_max_len").with_message(Cow::Owned(format!(
                "`claims` must not exceed {CLIENT_CLAIMS_MAX_LEN} serialized characters"
            ))),
        );
    }

    Ok(())
}

#[inline]
pub fn validate_vec_challenge(value: &[String]) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new(
            "'challenges' cannot be empty when provided",
        ));
    }
    for v in value {
        if !RE_CODE_CHALLENGE_METHOD.is_match(v) {
            return Err(ValidationError::new("^(plain|S256)$"));
        }
    }
    Ok(())
}

#[inline]
pub fn validate_vec_contact(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_CONTACT.is_match(v) {
            return Err(ValidationError::new("^[a-zA-Z0-9\\+.@/-]{0,48}$"));
        }
    }
    Ok(())
}

/// Strict grant-type validation for admin- and bootstrap-managed clients
/// (dynamic client registration, `UpdateClientRequest`, bootstrap config): the advertised
/// grant types are stored verbatim as the client's enabled flows, so an unknown/unsupported
/// one is rejected up front rather than silently persisted as a dead flow. The single
/// exception is the ephemeral (CIMD) path, which can opt into stripping unknown grant types
/// via `ephemeral_clients.ignore_unknown_auth_flows` (see `Client::ephemeral_from_url`).
#[inline]
pub fn validate_vec_grant_types(value: &[String]) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new(
            "'flows_enabled' cannot be empty when provided",
        ));
    }
    for v in value {
        if !RE_GRANT_TYPES.is_match(v) {
            return Err(ValidationError::new(
                "^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$",
            ));
        }
    }
    Ok(())
}

#[inline]
pub fn validate_vec_linux_hostname(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_LINUX_HOSTNAME.is_match(v) {
            return Err(ValidationError::new(
                "^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$",
            ));
        }
    }
    Ok(())
}

#[inline]
pub fn validate_vec_origin(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_ORIGIN.get().unwrap().is_match(v) {
            return Err(ValidationError::new("^(http|https)://[a-z0-9.:-]+$"));
        }
    }
    Ok(())
}

#[inline]
pub fn validate_vec_uri(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_URI.is_match(v) {
            return Err(ValidationError::new("^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"));
        }
    }
    Ok(())
}

/// RFC 8707 resource indicators (client `allowed_resources` / `default_aud`). Same as a URI
/// but a fragment (`#`) is not allowed, since a resource indicator MUST be an absolute URI
/// without a fragment (RFC 8707 §2). Matches the request-side `resource` validation.
#[inline]
pub fn validate_vec_resource(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_RESOURCE.is_match(v) {
            return Err(ValidationError::new("^[a-zA-Z0-9,.:/_\\-&?=~!$'()*+%@]+$"));
        }
    }
    Ok(())
}

/// Grant-type validation for ephemeral (CIMD) client documents. Strict by default: an
/// advertised grant type Rauthy does not support is rejected. An operator can opt into
/// accepting such a document by enabling `ephemeral_clients.ignore_unknown_auth_flows`,
/// which strips the unknown grant types in `Client::ephemeral_from_url` *before* this
/// validation runs, so the sanitized list passes here.
#[inline]
pub fn validate_vec_grant_type(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_GRANT_TYPES.is_match(v) {
            return Err(ValidationError::new(
                "^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$",
            ));
        }
    }
    Ok(())
}

// validate_vec_groups, _roles and _scopes do the same thing but are 3 functions just to
// be clear in the validation fields above that it does not create confusion, even if they
// all use the same `RE_GROUPS` regex.
#[inline]
pub fn validate_vec_groups(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_GROUPS.is_match(v) {
            return Err(ValidationError::new("^[a-zA-Z0-9-_/,:*\\s]{2,64}$"));
        }
    }
    Ok(())
}

#[inline]
pub fn validate_vec_roles(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_ROLES_SCOPES.is_match(v) {
            return Err(ValidationError::new("^[a-zA-Z0-9-_/,:*.]{2,64}$"));
        }
    }
    Ok(())
}

#[inline]
pub fn validate_vec_scopes(value: &[String]) -> Result<(), ValidationError> {
    for v in value {
        if !RE_ROLES_SCOPES.is_match(v) {
            return Err(ValidationError::new("^[a-zA-Z0-9-_/,:*.]{2,64}$"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn client_claims_must_be_an_object() {
        assert!(validate_claims(&json!({"tenant": "acme", "tier": 3})).is_ok());
        assert!(validate_claims(&json!({})).is_ok());

        // anything that is not a JSON object is rejected
        assert!(validate_claims(&json!("a string")).is_err());
        assert!(validate_claims(&json!(42)).is_err());
        assert!(validate_claims(&json!(true)).is_err());
        assert!(validate_claims(&json!(["a", "b"])).is_err());
        assert!(validate_claims(&serde_json::Value::Null).is_err());
    }

    #[test]
    fn grant_type_validators_reject_unknown_by_default() {
        // A spec-valid client (e.g. claude.ai) may advertise grant types Rauthy does not
        // implement. By default BOTH validators reject them - DCR/admin/bootstrap store the
        // list verbatim, and the ephemeral path only accepts unknown grants after they are
        // stripped upstream (gated by `ephemeral_clients.ignore_unknown_auth_flows`).
        let with_unknown = [
            "authorization_code",
            "refresh_token",
            "urn:ietf:params:oauth:grant-type:jwt-bearer",
        ]
        .map(String::from);
        assert!(validate_vec_grant_types(&with_unknown).is_err());
        assert!(validate_vec_grant_type(&with_unknown).is_err());

        // An all-supported list passes both validators.
        let all_known = ["authorization_code", "refresh_token"].map(String::from);
        assert!(validate_vec_grant_types(&all_known).is_ok());
        assert!(validate_vec_grant_type(&all_known).is_ok());

        // The plural validator rejects an explicitly empty list; the singular has no
        // empty-check (an ephemeral document may legitimately omit grant_types).
        let empty: [String; 0] = [];
        assert!(validate_vec_grant_types(&empty).is_err());
        assert!(validate_vec_grant_type(&empty).is_ok());
    }

    #[test]
    fn client_claims_respect_the_size_cap() {
        // a value just under the 1024 serialized-byte cap is accepted
        let ok = json!({ "k": "a".repeat(1000) });
        assert!(serde_json::to_vec(&ok).unwrap().len() <= 1024);
        assert!(validate_claims(&ok).is_ok());

        // a value over the cap is rejected
        let too_big = json!({ "k": "a".repeat(1024) });
        assert!(serde_json::to_vec(&too_big).unwrap().len() > 1024);
        assert!(validate_claims(&too_big).is_err());
    }
}
