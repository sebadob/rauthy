use crate::auth;
use actix_web::web;
use rauthy_common::utils::base64_url_no_pad_encode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::scopes::Scope;
use rauthy_models::entity::user_attr::UserAttrValueEntity;
use rauthy_models::entity::users::User;
use rauthy_models::JwtTokenType;
use ring::digest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use utoipa::ToSchema;

pub enum AtHashAlg {
    Sha256,
    Sha384,
    Sha512,
}

impl TryFrom<&str> for AtHashAlg {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let slf = match value {
            "RS256" => Self::Sha256,
            "RS384" => Self::Sha384,
            "RS512" => Self::Sha512,
            "EdDSA" => Self::Sha512,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "Cannot build AtHashAlg from value".to_string(),
                ));
            }
        };
        Ok(slf)
    }
}

pub struct AtHash(pub String);

impl AtHash {
    pub fn build(access_token: &[u8], alg: AtHashAlg) -> Self {
        let hash = match alg {
            AtHashAlg::Sha256 => digest::digest(&digest::SHA256, access_token),
            AtHashAlg::Sha384 => digest::digest(&digest::SHA384, access_token),
            AtHashAlg::Sha512 => digest::digest(&digest::SHA512, access_token),
        };
        let bytes = hash.as_ref();
        let (left_bits, _) = bytes.split_at(bytes.len() / 2);
        Self(base64_url_no_pad_encode(left_bits))
    }
}

#[derive(Debug, PartialEq)]
pub enum AuthCodeFlow {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCodeFlow {
    Yes(String),
    No,
}

#[derive(Clone)]
pub struct DpopFingerprint(pub String);

pub struct TokenNonce(pub String);

/// Contains the scopes as a single String separated by `\s`
pub struct TokenScopes(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenSet {
    pub access_token: String,
    pub token_type: JwtTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    pub expires_in: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl TokenSet {
    pub async fn for_client_credentials(
        data: &web::Data<AppState>,
        client: &Client,
        dpop_fingerprint: Option<DpopFingerprint>,
    ) -> Result<Self, ErrorResponse> {
        let token_type = if dpop_fingerprint.is_some() {
            JwtTokenType::DPoP
        } else {
            JwtTokenType::Bearer
        };
        let access_token = auth::build_access_token(
            None,
            data,
            client,
            dpop_fingerprint,
            client.access_token_lifetime as i64,
            None,
            None,
            DeviceCodeFlow::No,
        )
        .await?;

        Ok(Self {
            access_token,
            token_type,
            id_token: None,
            expires_in: client.access_token_lifetime,
            refresh_token: None,
        })
    }

    // too many arguments is not an issue - params cannot be mistaken because of enum wrappers
    #[allow(clippy::too_many_arguments)]
    pub async fn from_user(
        user: &User,
        data: &web::Data<AppState>,
        client: &Client,
        dpop_fingerprint: Option<DpopFingerprint>,
        nonce: Option<TokenNonce>,
        scopes: Option<TokenScopes>,
        auth_code_flow: AuthCodeFlow,
        device_code_flow: DeviceCodeFlow,
    ) -> Result<Self, ErrorResponse> {
        let scopes = scopes.map(|s| s.0);
        let scope = if let Some(s) = &scopes {
            s.clone()
        } else {
            client.default_scopes.clone().replace(',', " ")
        };

        // check for any non-custom scopes and prepare data
        let cust = Scope::extract_custom(&scope);

        let scps;
        let attrs;
        let (customs_access, customs_id) = if !cust.is_empty() {
            scps = Some(Scope::find_all(data).await?);

            let mut customs_access = Vec::with_capacity(cust.len());
            let mut customs_id = Vec::with_capacity(cust.len());

            for s in scps.as_ref().unwrap() {
                if cust.contains(s.name.as_str()) {
                    if s.attr_include_access.is_some() {
                        customs_access.push(s);
                    }
                    if s.attr_include_id.is_some() {
                        customs_id.push(s);
                    }
                }
            }

            // if there was any custom mapping, we need the additional user attributes
            attrs = if !customs_access.is_empty() || !customs_id.is_empty() {
                let attrs = UserAttrValueEntity::find_for_user(data, &user.id).await?;
                let mut res = HashMap::with_capacity(attrs.len());
                attrs.iter().for_each(|a| {
                    res.insert(a.key.clone(), a.value.clone());
                });
                Some(res)
            } else {
                None
            };

            // prepare the result
            let access = if customs_access.is_empty() {
                None
            } else {
                Some((customs_access, &attrs))
            };
            let id = if customs_id.is_empty() {
                None
            } else {
                Some((customs_id, &attrs))
            };

            (access, id)
        } else {
            (None, None)
        };

        // set the correct lifetime
        let lifetime = if let Some(ts) = user.user_expires {
            let now = OffsetDateTime::now_utc().unix_timestamp();
            let diff = ts - now;
            if diff < 1 {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User has expired".to_string(),
                ));
            }

            let client_lt = client.access_token_lifetime.unsigned_abs() as i64;
            if client_lt < diff {
                client_lt
            } else {
                diff
            }
        } else {
            client.access_token_lifetime.unsigned_abs() as i64
        };

        let token_type = if dpop_fingerprint.is_some() {
            JwtTokenType::DPoP
        } else {
            JwtTokenType::Bearer
        };
        let access_token = auth::build_access_token(
            Some(user),
            data,
            client,
            dpop_fingerprint.clone(),
            lifetime,
            Some(TokenScopes(scope.clone())),
            customs_access,
            device_code_flow.clone(),
        )
        .await?;

        let at_hash = AtHash::build(
            access_token.as_bytes(),
            AtHashAlg::try_from(client.access_token_alg.as_str())?,
        );
        let id_token = auth::build_id_token(
            user,
            data,
            client,
            dpop_fingerprint.clone(),
            at_hash,
            lifetime,
            nonce,
            &scope,
            customs_id,
            auth_code_flow,
        )
        .await?;
        let refresh_token = if client.allow_refresh_token() {
            Some(
                auth::build_refresh_token(
                    user,
                    data,
                    dpop_fingerprint,
                    client,
                    lifetime,
                    scopes.map(TokenScopes),
                    user.has_webauthn_enabled(),
                    device_code_flow,
                )
                .await?,
            )
        } else {
            None
        };

        Ok(Self {
            access_token,
            token_type,
            id_token: Some(id_token),
            expires_in: client.access_token_lifetime,
            refresh_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_hash() {
        let ref_token =
            b"YmJiZTAwYmYtMzgyOC00NzhkLTkyOTItNjJjNDM3MGYzOWIy9sFhvH8K_x8UIHj1osisS57f5DduL";

        let sha256 = AtHash::build(ref_token, AtHashAlg::Sha256);
        assert_eq!(&sha256.0, "xsZZrUssMXjL3FBlzoSh2g");

        let sha384 = AtHash::build(ref_token, AtHashAlg::Sha384);
        assert_eq!(&sha384.0, "adt46pcdiB-l6eTNifgoVM-5AIJAxq84");

        let sha512 = AtHash::build(ref_token, AtHashAlg::Sha512);
        assert_eq!(&sha512.0, "p2LHG4H-8pYDc0hyVOo3iIHvZJUqe9tbj3jESOuXbkY");
    }
}
