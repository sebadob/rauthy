use actix_web::web;
use chrono::Utc;
use jwt_simple::algorithms::{EdDSAKeyPairLike, RSAKeyPairLike};
use jwt_simple::claims::Claims;
use jwt_simple::prelude::{coarsetime, UnixTimeStamp};
use rauthy_api_types::oidc::JktClaim;
use rauthy_common::constants::{
    DEVICE_GRANT_REFRESH_TOKEN_LIFETIME, ENABLE_SOLID_AUD, ENABLE_WEB_ID, REFRESH_TOKEN_LIFETIME,
};
use rauthy_common::utils::base64_url_no_pad_encode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_models::entity::scopes::Scope;
use rauthy_models::entity::user_attr::UserAttrValueEntity;
use rauthy_models::entity::users::User;
use rauthy_models::entity::users_values::UserValues;
use rauthy_models::entity::webids::WebId;
use rauthy_models::{
    sign_jwt, AddressClaim, JwtAccessClaims, JwtAmrValue, JwtIdClaims, JwtRefreshClaims,
    JwtTokenType,
};
use ring::digest;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::str::FromStr;
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

#[derive(Debug, Clone)]
pub struct AuthTime(i64);

impl AuthTime {
    pub fn now() -> Self {
        Self(Utc::now().timestamp())
    }

    pub fn given(ts: i64) -> Self {
        Self(ts)
    }

    pub fn get(&self) -> i64 {
        self.0
    }
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
    /// Builds the access token for a user after all validation has been successful
    // too many arguments is not an issue - params cannot be mistaken because of enum wrappers
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub async fn build_access_token(
        user: Option<&User>,
        data: &web::Data<AppState>,
        client: &Client,
        dpop_fingerprint: Option<DpopFingerprint>,
        lifetime: i64,
        scope: Option<TokenScopes>,
        scope_customs: Option<(Vec<&Scope>, &Option<HashMap<String, Vec<u8>>>)>,
        device_code_flow: DeviceCodeFlow,
    ) -> Result<String, ErrorResponse> {
        let did = match device_code_flow {
            DeviceCodeFlow::Yes(did) => Some(did),
            DeviceCodeFlow::No => None,
        };
        let mut custom_claims = JwtAccessClaims {
            typ: JwtTokenType::Bearer,
            azp: client.id.to_string(),
            scope: scope
                .map(|s| s.0)
                .unwrap_or_else(|| client.default_scopes.clone().replace(',', " ")),
            allowed_origins: None,
            did,
            email: None,
            preferred_username: None,
            roles: None,
            groups: None,
            cnf: dpop_fingerprint.map(|jkt| JktClaim { jkt: jkt.0 }),
            custom: None,
        };

        // add user specific claims if available
        let sub = if let Some(user) = user {
            custom_claims.preferred_username = Some(user.email.clone());
            custom_claims.roles = Some(user.get_roles());

            if custom_claims.scope.contains("email") {
                custom_claims.email = Some(user.email.clone());
            }

            if custom_claims.scope.contains("groups") {
                custom_claims.groups = Some(user.get_groups());
            }

            Some(&user.id)
        } else {
            None
        };

        if let Some((cust, user_attrs)) = scope_customs {
            let user_attrs = user_attrs.as_ref().unwrap();
            let mut attr = HashMap::with_capacity(cust.len());
            for c in cust {
                if let Some(csv) = &c.attr_include_access {
                    let scopes = csv.split(',');
                    for cust_name in scopes {
                        if let Some(value) = user_attrs.get(cust_name) {
                            let json = serde_json::from_slice(value.as_slice())
                                .expect("Converting cust user id attr to json");
                            attr.insert(cust_name.to_string(), json);
                        };
                    }
                }
            }
            if !attr.is_empty() {
                custom_claims.custom = Some(attr);
            }
        }

        let mut claims = Claims::with_custom_claims(
            custom_claims,
            coarsetime::Duration::from_secs(lifetime as u64),
        )
        .with_issuer(data.issuer.clone())
        .with_audience(client.id.to_string());

        if let Some(sub) = sub {
            claims = claims.with_subject(sub);
        }

        // sign the token
        let key_pair_alg = JwkKeyPairAlg::from_str(&client.access_token_alg)?;
        let kp = JwkKeyPair::find_latest(data, key_pair_alg).await?;
        sign_jwt!(kp, claims)
    }

    /// Builds the id token for a user after all validation has been successful
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub async fn build_id_token(
        user: &User,
        data: &web::Data<AppState>,
        client: &Client,
        auth_time: AuthTime,
        dpop_fingerprint: Option<DpopFingerprint>,
        at_hash: AtHash,
        lifetime: i64,
        nonce: Option<TokenNonce>,
        scope: &str,
        scope_customs: Option<(Vec<&Scope>, &Option<HashMap<String, Vec<u8>>>)>,
        auth_code_flow: AuthCodeFlow,
    ) -> Result<String, ErrorResponse> {
        let amr = if user.has_webauthn_enabled() && auth_code_flow == AuthCodeFlow::Yes {
            JwtAmrValue::Mfa.to_string()
        } else {
            JwtAmrValue::Pwd.to_string()
        };

        let webid =
            (*ENABLE_WEB_ID && scope.contains("webid")).then(|| WebId::resolve_webid_uri(&user.id));

        let mut custom_claims = JwtIdClaims {
            azp: client.id.clone(),
            typ: JwtTokenType::Id,
            amr: vec![amr],
            auth_time: auth_time.get(),
            at_hash: at_hash.0,
            preferred_username: user.email.clone(),
            email: None,
            email_verified: None,
            given_name: None,
            family_name: None,
            address: None,
            birthdate: None,
            locale: None,
            phone: None,
            roles: user.get_roles(),
            groups: None,
            cnf: dpop_fingerprint.map(|jkt| JktClaim { jkt: jkt.0 }),
            custom: None,
            webid,
        };

        let mut user_values = None;
        let mut user_values_fetched = false;

        if scope.contains("email") {
            custom_claims.email = Some(user.email.clone());
            custom_claims.email_verified = Some(user.email_verified);
        }

        if scope.contains("profile") {
            custom_claims.given_name = Some(user.given_name.clone());
            custom_claims.family_name = Some(user.family_name.clone());
            custom_claims.locale = Some(user.language.to_string());

            user_values = UserValues::find(data, &user.id).await?;
            user_values_fetched = true;

            if let Some(values) = &user_values {
                if let Some(birthdate) = &values.birthdate {
                    custom_claims.birthdate = Some(birthdate.clone());
                }
            }
        }

        if scope.contains("address") {
            if !user_values_fetched {
                user_values = UserValues::find(data, &user.id).await?;
                user_values_fetched = true;
            }

            if let Some(values) = &user_values {
                custom_claims.address = AddressClaim::try_build(user, values);
            }
        }

        if scope.contains("phone") {
            if !user_values_fetched {
                user_values = UserValues::find(data, &user.id).await?;
                // user_values_fetched = true;
            }

            if let Some(values) = &user_values {
                if let Some(phone) = &values.phone {
                    custom_claims.phone = Some(phone.clone());
                }
            }
        }

        if scope.contains("groups") {
            custom_claims.groups = Some(user.get_groups());
        }

        if let Some((cust, user_attrs)) = scope_customs {
            let user_attrs = user_attrs.as_ref().unwrap();
            let mut attr = HashMap::with_capacity(cust.len());
            for c in cust {
                if let Some(csv) = &c.attr_include_id {
                    let scopes = csv.split(',');
                    for cust_name in scopes {
                        if let Some(value) = user_attrs.get(cust_name) {
                            let json = serde_json::from_slice(value.as_slice())
                                .expect("Converting cust user id attr to json");
                            attr.insert(cust_name.to_string(), json);
                        };
                    }
                }
            }
            if !attr.is_empty() {
                custom_claims.custom = Some(attr);
            }
        }

        let mut claims = Claims::with_custom_claims(
            custom_claims,
            coarsetime::Duration::from_secs(lifetime as u64),
        )
        .with_subject(user.id.clone())
        .with_issuer(data.issuer.clone());

        // TODO should we maybe always include the "solid" claim here depending on if a webid exists?
        // like it is now, static clients would never include this claim, even though they might need it
        if client.is_ephemeral() && *ENABLE_SOLID_AUD {
            let mut aud = HashSet::with_capacity(2);
            aud.insert("solid".to_string());
            aud.insert(client.id.to_string());
            claims = claims.with_audiences(aud);
        } else {
            claims = claims.with_audience(client.id.to_string());
        }

        if let Some(nonce) = nonce {
            claims = claims.with_nonce(nonce.0);
        }

        // sign the token
        let key_pair_alg = JwkKeyPairAlg::from_str(&client.id_token_alg)?;
        let kp = JwkKeyPair::find_latest(data, key_pair_alg).await?;

        sign_jwt!(kp, claims)
    }

    /// Builds the refresh token for a user after all validation has been successful
    #[allow(clippy::too_many_arguments)]
    pub async fn build_refresh_token(
        user: &User,
        data: &web::Data<AppState>,
        dpop_fingerprint: Option<DpopFingerprint>,
        client: &Client,
        auth_time: AuthTime,
        access_token_lifetime: i64,
        scope: Option<TokenScopes>,
        is_mfa: bool,
        device_code_flow: DeviceCodeFlow,
    ) -> Result<String, ErrorResponse> {
        let did = if let DeviceCodeFlow::Yes(device_id) = device_code_flow {
            Some(device_id)
        } else {
            None
        };

        let custom_claims = JwtRefreshClaims {
            azp: client.id.clone(),
            typ: JwtTokenType::Refresh,
            uid: user.id.clone(),
            auth_time: Some(auth_time.get()),
            cnf: dpop_fingerprint.map(|jkt| JktClaim { jkt: jkt.0 }),
            did: did.clone(),
        };

        let nbf = Utc::now().add(chrono::Duration::seconds(access_token_lifetime - 60));
        let nbf_unix = UnixTimeStamp::from_secs(nbf.timestamp() as u64);

        let claims =
            Claims::with_custom_claims(custom_claims, coarsetime::Duration::from_hours(48))
                .with_issuer(data.issuer.clone())
                .invalid_before(nbf_unix)
                .with_audience(client.id.to_string());

        // sign the token
        let token = {
            let kp = JwkKeyPair::find_latest(data, JwkKeyPairAlg::default()).await?;
            sign_jwt!(kp, claims)
        }?;

        // only save the last 50 characters for validation
        let validation_string = String::from(&token).split_off(token.len() - 49);

        if let Some(device_id) = did {
            let exp = nbf.add(chrono::Duration::hours(
                *DEVICE_GRANT_REFRESH_TOKEN_LIFETIME as i64,
            ));
            RefreshTokenDevice::create(
                data,
                validation_string,
                device_id,
                user.id.clone(),
                nbf,
                exp,
                scope.map(|s| s.0),
            )
            .await?;
        } else {
            let exp = nbf.add(chrono::Duration::hours(*REFRESH_TOKEN_LIFETIME as i64));
            RefreshToken::create(
                data,
                validation_string,
                user.id.clone(),
                nbf,
                exp,
                scope.map(|s| s.0),
                is_mfa,
            )
            .await?;
        }

        Ok(token)
    }

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
        let access_token = Self::build_access_token(
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
        auth_time: AuthTime,
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
        let access_token = Self::build_access_token(
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
        let id_token = Self::build_id_token(
            user,
            data,
            client,
            auth_time.clone(),
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
                Self::build_refresh_token(
                    user,
                    data,
                    dpop_fingerprint,
                    client,
                    auth_time,
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
