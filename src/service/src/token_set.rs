use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use rauthy_api_types::oidc::JktClaim;
use rauthy_common::utils::base64_url_no_pad_encode;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::issued_tokens::IssuedToken;
use rauthy_data::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_data::entity::scopes::Scope;
use rauthy_data::entity::user_attr::UserAttrValueEntity;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_data::entity::webids::WebId;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{
    JwtAccessClaims, JwtAmrValue, JwtCommonClaims, JwtIdClaims, JwtTokenType,
};
use rauthy_jwt::token::JwtToken;
use ring::digest;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use utoipa::ToSchema;

pub struct AccessTokenJti(String);

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
    #[inline]
    pub fn now() -> Self {
        Self(Utc::now().timestamp())
    }

    #[inline]
    pub fn given(ts: i64) -> Self {
        Self(ts)
    }

    #[inline]
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
#[derive(Clone)]
pub struct SessionId(pub String);
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
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub async fn build_access_token(
        user: Option<&User>,
        client: &Client,
        dpop_fingerprint: Option<DpopFingerprint>,
        lifetime: i64,
        scope: Option<TokenScopes>,
        scope_customs: Option<(Vec<&Scope>, &Option<HashMap<String, Vec<u8>>>)>,
        sid: Option<SessionId>,
        device_code_flow: DeviceCodeFlow,
    ) -> Result<(AccessTokenJti, String), ErrorResponse> {
        let did = match device_code_flow {
            DeviceCodeFlow::Yes(did) => Some(did),
            DeviceCodeFlow::No => None,
        };
        let scope = scope
            .map(|s| Cow::from(s.0))
            .unwrap_or_else(|| Cow::from(client.default_scopes.replace(',', " ")));

        let email = if scope.contains("email") {
            user.as_ref().map(|u| u.email.as_str())
        } else {
            None
        };
        let roles = user.map(|u| u.roles_iter().collect());
        let groups = if scope.contains("groups") {
            user.map(|u| u.groups_iter().collect())
        } else {
            None
        };

        let user_id = user.map(|u| u.id.as_str());
        let now = Utc::now().timestamp();
        let exp = now + lifetime;

        let issued_token =
            IssuedToken::create(user_id, did.as_deref(), sid.map(|sid| sid.0), exp).await?;

        let sub = if let Some(user) = user {
            Some(user.id.as_str())
        } else if RauthyConfig::get().vars.access.client_credentials_map_sub {
            Some(client.id.as_str())
        } else {
            None
        };

        let mut claims_new_impl = JwtAccessClaims {
            common: JwtCommonClaims {
                iat: now,
                nbf: now,
                exp,
                iss: &RauthyConfig::get().issuer,
                jti: Some(&issued_token.jti),
                aud: Cow::Borrowed(client.id.as_str()),
                sub,
                typ: JwtTokenType::Bearer,
                azp: &client.id,
                scope: Some(scope),
                did: did.as_deref(),
                cnf: dpop_fingerprint
                    .as_ref()
                    .map(|jkt| JktClaim { jkt: &jkt.0 }),
            },
            allowed_origins: None,
            email,
            roles,
            groups,
            custom: None,
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
                claims_new_impl.custom = Some(attr.clone());
            }
        }

        let key_pair_alg = JwkKeyPairAlg::from_str(&client.access_token_alg)?;
        let kp = JwkKeyPair::find_latest(key_pair_alg).await?;
        let token = JwtToken::build(&kp, &claims_new_impl)?;

        Ok((AccessTokenJti(issued_token.jti), token))
    }

    /// Builds the id token for a user after all validation has been successful
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub async fn build_id_token(
        user: &User,
        client: &Client,
        auth_time: AuthTime,
        dpop_fingerprint: Option<DpopFingerprint>,
        at_hash: AtHash,
        lifetime: i64,
        nonce: Option<TokenNonce>,
        scope: &str,
        scope_customs: Option<(Vec<&Scope>, &Option<HashMap<String, Vec<u8>>>)>,
        sid: Option<SessionId>,
        auth_code_flow: AuthCodeFlow,
    ) -> Result<String, ErrorResponse> {
        let config = RauthyConfig::get();

        let amr = if user.has_webauthn_enabled() && auth_code_flow == AuthCodeFlow::Yes {
            JwtAmrValue::Mfa.as_str()
        } else {
            JwtAmrValue::Pwd.as_str()
        };
        let aud = if client.is_ephemeral() && config.vars.ephemeral_clients.enable_solid_aud {
            Cow::from(format!("[\"{}\",\"solid\"]", client.id))
        } else {
            Cow::Borrowed(client.id.as_str())
        };

        let user_values = UserValues::find(&user.id).await?;

        let webid = (config.vars.ephemeral_clients.enable_web_id && scope.contains("webid"))
            .then(|| Cow::from(WebId::resolve_webid_uri(&user.id)));

        let now = Utc::now().timestamp();
        let mut claims = JwtIdClaims {
            common: JwtCommonClaims {
                iat: now,
                nbf: now,
                exp: now + lifetime,
                iss: &config.issuer,
                jti: None,
                aud,
                sub: Some(user.id.as_str()),
                typ: JwtTokenType::Id,
                azp: &client.id,
                scope: Some(Cow::Borrowed(scope)),
                did: None,
                cnf: dpop_fingerprint
                    .as_ref()
                    .map(|jkt| JktClaim { jkt: &jkt.0 }),
            },
            amr: vec![amr],
            auth_time: auth_time.get(),
            at_hash: at_hash.0.as_str(),
            sid: sid.as_ref().map(|sid| sid.0.as_str()),
            email: None,
            email_verified: None,
            preferred_username: None,
            given_name: None,
            family_name: None,
            address: None,
            birthdate: None,
            picture: None,
            locale: None,
            nonce: nonce.as_ref().map(|n| n.0.as_str()),
            phone: None,
            roles: user.get_roles(),
            groups: None,
            custom: None,
            webid,
            zoneinfo: None,
        };

        if scope.contains("email") {
            claims.email = Some(user.email.as_str());
            claims.email_verified = Some(user.email_verified);
        }
        if scope.contains("profile") {
            claims.given_name = Some(user.given_name.as_str());
            claims.family_name = user.family_name.as_deref();
            claims.locale = Some(user.language.as_str());

            if let Some(uv) = &user_values {
                if let Some(username) = &uv.preferred_username {
                    claims.preferred_username = Some(username);
                }

                if let Some(birthdate) = &uv.birthdate {
                    claims.birthdate = Some(birthdate.as_str());
                }

                if let Some(zone) = &uv.tz {
                    claims.zoneinfo = Some(zone)
                }
            }

            if config.vars.user_values.preferred_username.email_fallback
                && claims.preferred_username.is_none()
            {
                claims.preferred_username = Some(user.email.as_str());
            }

            claims.picture = user.picture_uri().map(Cow::from);
        }
        if scope.contains("address")
            && let Some(values) = &user_values
        {
            claims.address = rauthy_jwt::claims::AddressClaim::try_build(user, values);
        }
        if scope.contains("phone")
            && let Some(values) = &user_values
            && let Some(phone) = &values.phone
        {
            claims.phone = Some(phone.as_str());
        }
        if scope.contains("groups") {
            claims.groups = Some(user.get_groups());
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
                claims.custom = Some(attr);
            }
        }

        let key_pair_alg = JwkKeyPairAlg::from_str(&client.id_token_alg)?;
        let kp = JwkKeyPair::find_latest(key_pair_alg).await?;
        JwtToken::build(&kp, &claims)
    }

    /// Builds the refresh token for a user after all validation has been successful
    #[allow(clippy::too_many_arguments)]
    pub async fn build_refresh_token(
        user: &User,
        dpop_fingerprint: Option<DpopFingerprint>,
        client: &Client,
        auth_time: AuthTime,
        access_token_lifetime: i64,
        scope: Option<TokenScopes>,
        is_mfa: bool,
        device_code_flow: DeviceCodeFlow,
        sid: Option<SessionId>,
        jti: AccessTokenJti,
    ) -> Result<String, ErrorResponse> {
        let did = if let DeviceCodeFlow::Yes(device_id) = device_code_flow {
            Some(device_id)
        } else {
            None
        };

        let now = Utc::now().timestamp();
        let nbf = if RauthyConfig::get().vars.access.disable_refresh_token_nbf {
            now
        } else {
            // allow 60 second early usage
            now + access_token_lifetime - 60
        };
        let exp = if did.is_some() {
            nbf + 3600 * RauthyConfig::get().vars.device_grant.refresh_token_lifetime as i64
        } else {
            nbf + 3600 * RauthyConfig::get().vars.lifetimes.refresh_token_lifetime as i64
        };

        let token = {
            let claims = rauthy_jwt::claims::JwtRefreshClaims {
                common: JwtCommonClaims {
                    iat: now,
                    nbf,
                    exp,
                    iss: &RauthyConfig::get().issuer,
                    // jti is not really used for any validation, it just exists
                    // to bring a bit more randomness into the claims
                    jti: Some(&secure_random_alnum(8)),
                    aud: Cow::Borrowed(client.id.as_str()),
                    sub: None,
                    typ: JwtTokenType::Refresh,
                    azp: &client.id,
                    scope: None,
                    did: did.as_deref(),
                    cnf: dpop_fingerprint
                        .as_ref()
                        .map(|jkt| JktClaim { jkt: &jkt.0 }),
                },
                uid: &user.id,
                // Only Optional for backwards compatibility with older Rauthy versions and tokens.
                // Could be changed with v1.0.0 maybe.
                auth_time: Some(auth_time.get()),
            };

            let kp = JwkKeyPair::find_latest(JwkKeyPairAlg::default()).await?;
            JwtToken::build(&kp, &claims)?
        };

        // only save the last 50 characters for validation
        let validation_string = String::from(&token).split_off(token.len() - 49);

        if let Some(device_id) = did {
            RefreshTokenDevice::create(
                validation_string,
                device_id,
                user.id.clone(),
                nbf,
                exp,
                scope.map(|s| s.0),
                Some(jti.0),
            )
            .await?;
        } else {
            RefreshToken::create(
                validation_string,
                user.id.clone(),
                nbf,
                exp,
                scope.map(|s| s.0),
                is_mfa,
                sid.map(|s| s.0),
                Some(jti.0),
            )
            .await?;
        }

        Ok(token)
    }

    pub async fn for_client_credentials(
        client: &Client,
        dpop_fingerprint: Option<DpopFingerprint>,
    ) -> Result<Self, ErrorResponse> {
        let token_type = if dpop_fingerprint.is_some() {
            JwtTokenType::DPoP
        } else {
            JwtTokenType::Bearer
        };
        let (_jti, access_token) = Self::build_access_token(
            None,
            client,
            dpop_fingerprint,
            client.access_token_lifetime as i64,
            None,
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

    // too many arguments is not an issue - params cannot be mistaken because of typed wrappers
    #[allow(clippy::too_many_arguments)]
    pub async fn from_user(
        user: &User,
        client: &Client,
        auth_time: AuthTime,
        dpop_fingerprint: Option<DpopFingerprint>,
        nonce: Option<TokenNonce>,
        scopes: Option<TokenScopes>,
        sid: Option<SessionId>,
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
            scps = Some(Scope::find_all().await?);

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
                let attrs = UserAttrValueEntity::find_for_user_with_defaults(&user.id).await?;
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
            let now = Utc::now().timestamp();
            let diff = ts - now;
            if diff < 1 {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User has expired",
                ));
            }

            let client_lt = client.access_token_lifetime.unsigned_abs() as i64;
            if client_lt < diff { client_lt } else { diff }
        } else {
            client.access_token_lifetime.unsigned_abs() as i64
        };

        let token_type = if dpop_fingerprint.is_some() {
            JwtTokenType::DPoP
        } else {
            JwtTokenType::Bearer
        };
        let (jti, access_token) = Self::build_access_token(
            Some(user),
            client,
            dpop_fingerprint.clone(),
            lifetime,
            Some(TokenScopes(scope.clone())),
            customs_access,
            sid.clone(),
            device_code_flow.clone(),
        )
        .await?;

        let at_hash = AtHash::build(
            access_token.as_bytes(),
            AtHashAlg::try_from(client.access_token_alg.as_str())?,
        );
        let id_token = Self::build_id_token(
            user,
            client,
            auth_time.clone(),
            dpop_fingerprint.clone(),
            at_hash,
            lifetime,
            nonce,
            &scope,
            customs_id,
            sid.clone(),
            auth_code_flow,
        )
        .await?;
        let refresh_token = if client.allow_refresh_token() {
            Some(
                Self::build_refresh_token(
                    user,
                    dpop_fingerprint,
                    client,
                    auth_time,
                    lifetime,
                    scopes.map(TokenScopes),
                    user.has_webauthn_enabled(),
                    device_code_flow,
                    sid,
                    jti,
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
