use rauthy_api_types::users::{UpdateUserRequest, UserValuesRequest};
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_data::rauthy_config::{RauthyConfig, UserValueConfigValue, VarsUserValuesConfig};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::borrow::Cow;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug)]
pub struct UserValuesValidator<'a> {
    pub given_name: Option<&'a str>,
    pub family_name: Option<&'a str>,
    pub preferred_username: Option<&'a str>,
    pub user_values: &'a Option<UserValuesRequest>,
}

impl<'a> UserValuesValidator<'a> {
    /// `preferred_username` has its own endpoint, so pass the stored one.
    pub fn from_update_req(
        req: &'a UpdateUserRequest,
        preferred_username: Option<&'a str>,
    ) -> Self {
        Self {
            given_name: req.given_name.as_deref(),
            family_name: req.family_name.as_deref(),
            preferred_username,
            user_values: &req.user_values,
        }
    }

    pub fn validate(&self) -> Result<(), ErrorResponse> {
        self.validate_with(&RauthyConfig::get().vars.user_values)
    }

    /// Rejects an update only if it would make a currently valid user invalid. A user already
    /// missing a (newly) required value can still be edited, `revalidate_during_login` is the
    /// way to make them complete it.
    pub fn validate_update(&self, stored: &Self) -> Result<(), ErrorResponse> {
        self.validate_update_with(stored, &RauthyConfig::get().vars.user_values)
    }

    fn validate_update_with(
        &self,
        stored: &Self,
        config: &VarsUserValuesConfig,
    ) -> Result<(), ErrorResponse> {
        if stored.validate_with(config).is_ok() {
            self.validate_with(config)
        } else {
            self.validate_tz()
        }
    }

    fn validate_with(&self, config: &VarsUserValuesConfig) -> Result<(), ErrorResponse> {
        if config.given_name == UserValueConfigValue::Required
            && (self.given_name.is_none() || self.given_name == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'given_name' is required",
            ));
        }
        if config.family_name == UserValueConfigValue::Required
            && (self.family_name.is_none() || self.family_name == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'family_name' is required",
            ));
        }

        if config.preferred_username.preferred_username == UserValueConfigValue::Required
            && (self.preferred_username.is_none() || self.preferred_username == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'preferred_username' is required",
            ));
        }

        if let Some(username) = self.preferred_username
            && config
                .preferred_username
                .blacklist
                .contains(&Cow::from(username.to_lowercase()))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotAccepted,
                format!("username '{username}' is not available",),
            ));
        }

        // Run the required checks even when `user_values` is omitted entirely.
        let default_uv = UserValuesRequest::default();
        let uv = self.user_values.as_ref().unwrap_or(&default_uv);
        if config.birthdate == UserValueConfigValue::Required
            && (uv.birthdate.is_none() || uv.birthdate.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'birthdate' is required",
            ));
        }
        if config.street == UserValueConfigValue::Required
            && (uv.street.is_none() || uv.street.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'street' is required",
            ));
        }
        if config.zip == UserValueConfigValue::Required
            && (uv.zip.is_none() || uv.zip.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'zip' is required",
            ));
        }
        if config.city == UserValueConfigValue::Required
            && (uv.city.is_none() || uv.city.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'city' is required",
            ));
        }
        if config.country == UserValueConfigValue::Required
            && (uv.country.is_none() || uv.country.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'country' is required",
            ));
        }
        if config.phone == UserValueConfigValue::Required
            && (uv.phone.is_none() || uv.phone.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'phone' is required",
            ));
        }

        if config.tz == UserValueConfigValue::Required
            && (uv.tz.is_none() || uv.tz.as_deref() == Some(""))
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'tz' is required",
            ));
        }

        self.validate_tz()
    }

    fn validate_tz(&self) -> Result<(), ErrorResponse> {
        if let Some(tz) = self.user_values.as_ref().and_then(|uv| uv.tz.as_deref())
            && chrono_tz::Tz::from_str(tz).is_err()
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'tz' cannot be parsed",
            ));
        }
        Ok(())
    }

    #[inline(always)]
    pub async fn does_user_need_update(
        user: &User,
        client_id: &str,
    ) -> Result<bool, ErrorResponse> {
        if client_id == "rauthy" {
            return Ok(false);
        }

        if RauthyConfig::get().vars.user_values.revalidate_during_login {
            let uv = UserValues::find(&user.id).await?.unwrap_or_default();
            let user_values = Some(UserValuesRequest {
                birthdate: uv.birthdate,
                phone: uv.phone,
                street: uv.street,
                zip: uv.zip,
                city: uv.city,
                country: uv.country,
                tz: uv.tz,
            });
            let needs_update = UserValuesValidator {
                given_name: Some(&user.given_name),
                family_name: user.family_name.as_deref(),
                preferred_username: uv.preferred_username.as_deref(),
                user_values: &user_values,
            }
            .validate()
            .is_err();

            debug!(needs_update, "User needs to accept updated ToS");
            Ok(needs_update)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rauthy_data::rauthy_config::Vars;

    #[test]
    fn required_phone_is_enforced_when_user_values_is_omitted() {
        let mut config = Vars::default().user_values;
        config.phone = UserValueConfigValue::Required;

        let validator = UserValuesValidator {
            given_name: Some("A"),
            family_name: None,
            preferred_username: None,
            user_values: &None,
        };

        assert!(validator.validate_with(&config).is_err());
    }

    #[test]
    fn required_value_cleared_to_empty_string_is_rejected() {
        let mut config = Vars::default().user_values;
        config.street = UserValueConfigValue::Required;

        let user_values = Some(UserValuesRequest {
            street: Some(String::new()),
            ..Default::default()
        });
        let validator = UserValuesValidator {
            given_name: Some("A"),
            family_name: None,
            preferred_username: None,
            user_values: &user_values,
        };

        assert!(validator.validate_with(&config).is_err());
    }

    fn values(street: Option<&str>) -> Option<UserValuesRequest> {
        Some(UserValuesRequest {
            street: street.map(String::from),
            ..Default::default()
        })
    }

    fn validator<'a>(
        preferred_username: Option<&'a str>,
        user_values: &'a Option<UserValuesRequest>,
    ) -> UserValuesValidator<'a> {
        UserValuesValidator {
            given_name: Some("A"),
            family_name: None,
            preferred_username,
            user_values,
        }
    }

    #[test]
    fn required_preferred_username_passes_with_the_stored_one() {
        let mut config = Vars::default().user_values;
        config.preferred_username.preferred_username = UserValueConfigValue::Required;
        let uv = values(Some("Main St"));

        let stored = validator(Some("alfred"), &uv);
        let updated = validator(Some("alfred"), &uv);
        assert!(updated.validate_update_with(&stored, &config).is_ok());
        assert!(updated.validate_with(&config).is_ok());
        assert!(validator(None, &uv).validate_with(&config).is_err());
    }

    #[test]
    fn valid_user_cannot_clear_a_required_value() {
        let mut config = Vars::default().user_values;
        config.street = UserValueConfigValue::Required;
        let stored_uv = values(Some("Main St"));
        let cleared = values(Some(""));
        let deleted = None;

        let stored = validator(None, &stored_uv);
        assert!(
            validator(None, &cleared)
                .validate_update_with(&stored, &config)
                .is_err()
        );
        assert!(
            validator(None, &deleted)
                .validate_update_with(&stored, &config)
                .is_err()
        );
    }

    #[test]
    fn already_invalid_user_can_still_be_updated() {
        let mut config = Vars::default().user_values;
        config.street = UserValueConfigValue::Required;
        let missing = values(None);

        let stored = validator(None, &missing);
        assert!(
            validator(None, &missing)
                .validate_update_with(&stored, &config)
                .is_ok()
        );

        let bad_tz = Some(UserValuesRequest {
            tz: Some("Not/AZone".to_string()),
            ..Default::default()
        });
        assert!(
            validator(None, &bad_tz)
                .validate_update_with(&stored, &config)
                .is_err()
        );
    }
}
