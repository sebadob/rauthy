use crate::database::{Cache, DB};
use crate::entity::webauthn::auth_req::{WebauthnLoginReq, WebauthnServiceReq};
use crate::entity::webauthn::authenticate::WebauthnLoginToSAwaitCode;
use crate::rauthy_config::RauthyConfig;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderValue,
};
use actix_web::http::{StatusCode, header};
use actix_web::{HttpResponse, HttpResponseBuilder};
use rauthy_api_types::tos::ToSAwaitLoginResponse;
use rauthy_api_types::users::{ResidentKeyToken, WebauthnLoginFinishResponse};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebauthnData {
    pub code: String,
    // auth_state cannot be serialized directly with bincode -> no support for deserialize from any
    pub auth_state_json: String,
    pub data: WebauthnAdditionalData,
}

// CURD
impl WebauthnData {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::Webauthn, self.code.clone()).await?;
        Ok(())
    }

    pub async fn find_remove(code: String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::hql().get_remove(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.webauthn.data_exp as i64);
        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), &self, ttl)
            .await?;
        Ok(())
    }
}

// This is the data, that will be passed to the client as response to a successful MFA auth
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum WebauthnAdditionalData {
    Login(WebauthnLoginReq),
    Discover(Option<String>), // contains the Resident Key Token on success
    Service(WebauthnServiceReq),
    Test(String), // contains the User ID
    LoginToSAwait(WebauthnLoginToSAwaitCode),
}

impl WebauthnAdditionalData {
    pub fn into_response(self) -> HttpResponse {
        match self {
            Self::Login(login_req) => {
                let header_loc = (
                    header::LOCATION,
                    HeaderValue::from_str(&login_req.header_loc).unwrap(),
                );

                let mut builder = if login_req.needs_user_update {
                    HttpResponse::ResetContent()
                } else {
                    let mut builder = HttpResponse::Accepted();
                    builder.insert_header(header_loc);
                    builder
                };

                if let Some(value) = login_req.header_origin {
                    builder.insert_header((
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        HeaderValue::from_str(&value).unwrap(),
                    ));
                }

                if login_req.needs_user_update {
                    builder.finish()
                } else {
                    builder.json(WebauthnLoginFinishResponse {
                        loc: login_req.header_loc,
                    })
                }
            }

            Self::Discover(token) => {
                debug_assert!(token.is_some());
                HttpResponse::Accepted().json(ResidentKeyToken {
                    resident_key_token: token.unwrap_or_default(),
                })
            }

            Self::Service(svc_req) => HttpResponse::Accepted().json(svc_req),

            Self::Test(_) => HttpResponse::Accepted().finish(),

            Self::LoginToSAwait(tos_req) => {
                let mut resp = HttpResponseBuilder::new(StatusCode::from_u16(206).unwrap()).json(
                    &ToSAwaitLoginResponse {
                        tos_await_code: tos_req.await_code,
                        force_accept: None,
                    },
                );
                if let Some(origin) = tos_req.header_origin {
                    resp.headers_mut()
                        .insert(header::ORIGIN, HeaderValue::from_str(&origin).unwrap());
                    resp.headers_mut().insert(
                        ACCESS_CONTROL_ALLOW_METHODS,
                        HeaderValue::from_static("POST"),
                    );
                    resp.headers_mut().insert(
                        ACCESS_CONTROL_ALLOW_CREDENTIALS,
                        HeaderValue::from_static("true"),
                    );
                }
                resp
            }
        }
    }
}
