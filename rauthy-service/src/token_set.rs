use crate::auth;
use actix_web::web;
use rauthy_common::constants::{OFFLINE_TOKEN_LT, TOKEN_BEARER};
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::scopes::Scope;
use rauthy_models::entity::user_attr::UserAttrValueEntity;
use rauthy_models::entity::users::User;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenSet {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
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
    ) -> Result<Self, ErrorResponse> {
        let access_token = auth::build_access_token(
            None,
            data,
            client,
            client.access_token_lifetime as i64,
            None,
            None,
        )
        .await?;

        Ok(Self {
            access_token,
            token_type: Some(String::from(TOKEN_BEARER)),
            id_token: None,
            expires_in: client.access_token_lifetime,
            refresh_token: None,
        })
    }

    pub async fn from_user(
        user: &User,
        data: &web::Data<AppState>,
        client: &Client,
        nonce: Option<String>,
        scopes: Option<String>,
        is_auth_code_flow: bool,
    ) -> Result<Self, ErrorResponse> {
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
            // let (customs_access, customs_id, attrs) = if !cust.is_empty() {
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
        let lifetime = if scope.contains("offline_access") {
            *OFFLINE_TOKEN_LT
        } else {
            client.access_token_lifetime.unsigned_abs() as i64
        };

        let id_token = auth::build_id_token(
            user,
            data,
            client,
            lifetime,
            nonce,
            &scope,
            customs_id,
            is_auth_code_flow,
        )
        .await?;
        let access_token = auth::build_access_token(
            Some(user),
            data,
            client,
            lifetime,
            Some(scope),
            customs_access,
        )
        .await?;
        let refresh_token = if client.refresh_token {
            Some(
                auth::build_refresh_token(user, data, client, lifetime, scopes, is_auth_code_flow)
                    .await?,
            )
        } else {
            None
        };

        Ok(Self {
            access_token,
            token_type: None,
            id_token: Some(id_token),
            expires_in: client.access_token_lifetime,
            refresh_token,
        })
    }
}
