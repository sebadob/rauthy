use rauthy_common::regex::{
    RE_ATTR, RE_CODE_CHALLENGE_METHOD, RE_CONTACT, RE_GRANT_TYPES, RE_GROUPS, RE_LINUX_HOSTNAME,
    RE_ORIGIN, RE_ROLES_SCOPES, RE_URI,
};
use validator::ValidationError;

#[inline]
pub fn validate_vec_attr(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'validate_vec_attr' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_ATTR.is_match(v) {
                err = Some("^[a-z0-9-_/]{2,128}$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_challenge(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'challenges' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_CODE_CHALLENGE_METHOD.is_match(v) {
                err = Some("^(plain|S256)$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_contact(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_CONTACT.is_match(v) {
            err = Some("^[a-zA-Z0-9\\+.@/]{0,48}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_grant_types(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'flows_enabled' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_GRANT_TYPES.is_match(v) {
                err = Some("^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_linux_hostname(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    value.iter().for_each(|v| {
        if !RE_LINUX_HOSTNAME.is_match(v) {
            err = Some("^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$");
        }
    });

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_origin(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_ORIGIN.get().unwrap().is_match(v) {
            err = Some("^(http|https)://[a-z0-9.:-]+$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_uri(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_URI.is_match(v) {
            err = Some("^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_grant_type(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GRANT_TYPES.is_match(v) {
            err = Some("authorization_code|client_credentials|password|refresh_token");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

// validate_vec_groups, _roles and _scopes do the same thing but are 3 functions just to
// be clear in the validation fields above that it does not create confusion, even if they
// all use the same `RE_GROUPS` regex.
#[inline]
pub fn validate_vec_groups(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GROUPS.is_match(v) {
            err = Some("^[a-zA-Z0-9-_/,:*\\s]{2,64}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_roles(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_ROLES_SCOPES.is_match(v) {
            err = Some("^[a-zA-Z0-9-_/,:*.]{2,64}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[inline]
pub fn validate_vec_scopes(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_ROLES_SCOPES.is_match(v) {
            err = Some("^[a-zA-Z0-9-_/,:*.]{2,64}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}
