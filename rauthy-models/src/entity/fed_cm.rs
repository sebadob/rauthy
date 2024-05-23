use crate::app_state::AppState;
use crate::entity::colors::ColorEntity;
use crate::entity::users::User;
use actix_web::web;
use rauthy_common::constants::EMAIL_SUB_PREFIX;
use rauthy_common::error_response::ErrorResponse;
use serde::Serialize;
use std::sync::OnceLock;
use utoipa::ToSchema;

static IDP_CONFIG: OnceLock<FedCMIdPConfig> = OnceLock::new();

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMAccount {
    pub id: String,
    // The user’s full name.
    pub name: String,
    pub email: String,
    pub given_name: Option<String>,
    // URL for the account’s picture.
    pub picture: Option<String>,
    // A list of RPs (that gets matched against the requesting clientId) this account is already
    // registered with. Used in the request permission to sign-up to allow the IDP to control
    // whether to show the Privacy Policy and the Terms of Service.
    pub approved_clients: Vec<String>,
    // A list of strings which correspond to all of the login hints which match with this account.
    // An RP can use the loginHint to request that only an account matching a given value is shown
    // to the user.
    pub login_hints: Vec<String>,
    // A list of strings which correspond to all of the domain hints which match with this account.
    // An RP can use the domainHint to request that only an account matching a given value or
    // containing some domain hint is shown to the user.
    pub domain_hints: Vec<String>,
}

impl FedCMAccount {
    pub fn build(user: User, clients: Vec<String>) -> Self {
        let name = format!("{} {}", user.given_name, user.family_name);
        let login_hint = format!("login_hint={}", user.email);

        Self {
            id: user.id,
            name,
            email: user.email,
            given_name: Some(user.given_name),
            // Rauthy does not store user pictures
            picture: None,
            // TODO how should we decide which clients to return here? How to make this dynamic?
            // simply all of them? Or introduce a new flow to allow fedCm and filter?
            approved_clients: clients,
            login_hints: vec![login_hint, "state=fedcm".to_string()],
            // Rauthy does not use such a value
            domain_hints: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMAccounts {
    pub accounts: Vec<FedCMAccount>,
}

#[derive(Clone, Debug, Default, Serialize, ToSchema)]
pub struct FedCMClientMetadata {
    // A link to the RP's Privacy Policy.
    pub privacy_policy_url: String,
    // A link to the RP's Terms of Service.
    pub terms_of_service_url: String,
}

impl FedCMClientMetadata {
    pub fn new() -> Self {
        // Self {
        //     // Rauthy does not track any policy or ToS
        //     privacy_policy_url: String::default(),
        //     terms_of_service_url: String::default(),
        // }
        Self::default()
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMIdPIcon {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

impl FedCMIdPIcon {
    fn rauthy_logo(issuer: &str) -> Self {
        Self {
            url: format!("{}/clients/rauthy/logo", issuer),
            // Rauthy's default icon is an SVG which is fine according to the spec -> no size
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
        let colors = ColorEntity::find_rauthy(data).await?;
        let rauthy_icon = FedCMIdPIcon::rauthy_logo(&data.issuer);

        Ok(Self {
            // Background color for IDP-branded widgets such as buttons.
            background_color: Some(colors.act2),
            // color for text on IDP branded widgets.
            color: Some(colors.text),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_endpoint: Option<String>,
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
            login_url: format!("{}/account", iss),
            disconnect_endpoint: None,
            // disconnect_endpoint: format!("{}/{}/disconnect", iss, sub_path),
            branding,
        };

        let _ = IDP_CONFIG.set(slf);
        Ok(IDP_CONFIG.get().unwrap())
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum FedCMLoginStatus {
    LoggedIn,
    LoggedOut,
}

impl FedCMLoginStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FedCMLoginStatus::LoggedIn => "logged-in",
            FedCMLoginStatus::LoggedOut => "logged-out",
        }
    }

    pub fn into_header_pair(&self) -> (&str, &str) {
        ("Set-Login", self.as_str())
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct WebIdentity {
    pub provider_urls: Vec<String>,
}

impl WebIdentity {
    pub fn new(issuer: &str) -> Self {
        Self {
            provider_urls: vec![format!("{}/fed_cm/config", issuer)],
        }
    }
}
