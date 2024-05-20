use crate::app_state::AppState;
use crate::entity::colors::ColorEntity;
use actix_web::web;
use rauthy_common::constants::EMAIL_SUB_PREFIX;
use rauthy_common::error_response::ErrorResponse;
use serde::Serialize;
use std::sync::OnceLock;
use utoipa::ToSchema;

static IDP_CONFIG: OnceLock<FedCMIdPConfig> = OnceLock::new();

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMIdPIcon {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

impl FedCMIdPIcon {
    fn rauthy_logo(issuer: &str) -> Self {
        Self {
            // TODO this will return an SVG by default -> check if okay
            url: format!("{}/clients/rauthy/logo", issuer),
            size: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMIdPBranding {
    pub background_color: Option<String>,
    pub color: Option<String>,
    pub icons: Vec<FedCMIdPIcon>,
    pub name: Option<&'static str>,
}

impl FedCMIdPBranding {
    async fn new(data: &web::Data<AppState>) -> Result<Self, ErrorResponse> {
        let colors = ColorEntity::find_rauthy(&data).await?;
        let rauthy_icon = FedCMIdPIcon::rauthy_logo(&data.issuer);

        Ok(Self {
            background_color: Some(colors.bg),
            color: Some(colors.act2),
            icons: vec![rauthy_icon],
            name: Some(&*EMAIL_SUB_PREFIX),
        })
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMIdPConfig {
    pub accounts_endpoint: String,
    pub client_metadata_endpoint: String,
    pub id_assertion_endpoint: String,
    pub login_url: String,
    pub disconnect_endpoint: String,
    pub branding: FedCMIdPBranding,
}

impl FedCMIdPConfig {
    pub async fn get(data: &web::Data<AppState>) -> Result<&'static Self, ErrorResponse> {
        if let Some(slf) = IDP_CONFIG.get() {
            return Ok(slf);
        }

        let branding = FedCMIdPBranding::new(data).await?;

        let iss = &data.issuer;
        let sub_path = "fed_cm";

        let slf = Self {
            accounts_endpoint: format!("{}/{}/accounts", iss, sub_path),
            client_metadata_endpoint: format!("{}/{}/client_meta", iss, sub_path),
            id_assertion_endpoint: format!("{}/{}/token", iss, sub_path),
            // TODO where should be point this URL in case of Rauthy for it to make sense?
            login_url: format!("{}/{}/login", iss, sub_path),
            disconnect_endpoint: format!("{}/{}/disconnect", iss, sub_path),
            branding,
        };

        let _ = IDP_CONFIG.set(slf);
        Ok(IDP_CONFIG.get().unwrap())

        // let branding = FedCMIdPBranding::new(data).await?;
        //
        // let iss = &data.issuer;
        // let sub_path = "fed_cm";
        //
        // // TODO we could build all of this only once and then use a cached response each time
        // Ok(Self {
        //     accounts_endpoint: format!("{}/{}/accounts", iss, sub_path),
        //     client_metadata_endpoint: format!("{}/{}/client_meta", iss, sub_path),
        //     id_assertion_endpoint: format!("{}/{}/token", iss, sub_path),
        //     // TODO where should be point this URL in case of Rauthy for it to make sense?
        //     login_url: format!("{}/{}/login", iss, sub_path),
        //     disconnect_endpoint: format!("{}/{}/disconnect", iss, sub_path),
        //     branding,
        // })
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct WebIdentity {
    pub provider_urls: Vec<String>,
}

impl WebIdentity {
    pub fn new(issuer: &str) -> Self {
        Self {
            provider_urls: vec![format!("{}/fed_cm/config.json", issuer)],
        }
    }
}
