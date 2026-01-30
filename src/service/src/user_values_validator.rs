use rauthy_api_types::users::UserValuesRequest;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_data::rauthy_config::{RauthyConfig, UserValueConfigValue};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::str::FromStr;
use tracing::{debug, warn};

#[derive(Debug)]
pub struct UserValuesValidator<'a> {
    pub given_name: Option<&'a str>,
    pub family_name: Option<&'a str>,
    pub preferred_username: Option<&'a str>,
    pub user_values: &'a Option<UserValuesRequest>,
}

impl UserValuesValidator<'_> {
    pub fn validate(&self) -> Result<(), ErrorResponse> {
        warn!("Validating values {self:?}");
        let config = &RauthyConfig::get().vars.user_values;

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

        if let Some(uv) = self.user_values {
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
            if let Some(tz) = &uv.tz
                && chrono_tz::Tz::from_str(tz).is_err()
            {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "'tz' cannot be parsed",
                ));
            }
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
