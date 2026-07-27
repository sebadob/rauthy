use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
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
    pub fn build(user: User) -> Self {
        let name = user.email_recipient_name();
        let login_hint = format!("login_hint={}", user.email);

        Self {
            id: user.id,
            name,
            email: user.email,
            given_name: Some(user.given_name),
            // Rauthy does not store user pictures
            picture: None,
            // TODO how should we decide which clients to return here? How to make this dynamic?
            //  simply all of them? Or introduce a new flow to allow fedCm and filter?
            // approved_clients: clients,
            //  not sure if it produces errors and problems if we populate this value at all
            approved_clients: Vec::default(),
            login_hints: vec![login_hint, "state=fedcm".to_string()],
            domain_hints: vec![RauthyConfig::get().vars.server.pub_url.clone()],
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMAccounts {
    pub accounts: Vec<FedCMAccount>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMTokenResponse {
    pub token: String,
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
    fn rauthy_logo() -> Self {
        Self {
            url: format!("{}clients/rauthy/logo", RauthyConfig::get().issuer),
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
    async fn new() -> Result<Self, ErrorResponse> {
        let rauthy_icon = FedCMIdPIcon::rauthy_logo();

        // this is pretty inefficient, but FedCM is in experimental testing only anyway
        let css = ThemeCssFull::find_with_default("rauthy".to_string()).await?;
        let background_color = format!(
            "hsl({} {} {})",
            css.light.bg[0], css.light.bg[1], css.light.bg[2]
        );
        let color = format!(
            "hsl({} {} {})",
            css.light.text[0], css.light.text[1], css.light.text[2]
        );

        Ok(Self {
            // Background color for IDP-branded widgets such as buttons.
            background_color: Some(background_color),
            // color for text on IDP branded widgets.
            color: Some(color),
            icons: vec![rauthy_icon],
            name: Some(&RauthyConfig::get().vars.email.sub_prefix),
        })
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct FedCMIdPConfig {
    pub accounts_endpoint: &'static str,
    pub client_metadata_endpoint: &'static str,
    pub id_assertion_endpoint: &'static str,
    pub login_url: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_endpoint: Option<String>,
    pub branding: FedCMIdPBranding,
}

impl FedCMIdPConfig {
    pub async fn get() -> Result<&'static Self, ErrorResponse> {
        if let Some(slf) = IDP_CONFIG.get() {
            return Ok(slf);
        }

        let branding = FedCMIdPBranding::new().await?;
        let slf = Self {
            accounts_endpoint: "/auth/v1/fed_cm/accounts",
            client_metadata_endpoint: "/auth/v1/fed_cm/client_meta",
            id_assertion_endpoint: "/auth/v1/fed_cm/token",
            // TODO where should be point this URL in case of Rauthy for it to make sense?
            login_url: "/auth/v1/account",
            disconnect_endpoint: None,
            // disconnect_endpoint: format!("{}/{}/disconnect", iss, sub_path),
            branding,
        };

        let _ = IDP_CONFIG.set(slf);
        Ok(IDP_CONFIG.get().unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToSchema)]
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

    pub fn as_header_pair(&self) -> (&str, &str) {
        ("Set-Login", self.as_str())
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct WebIdentity {
    pub provider_urls: Vec<String>,
}

impl Default for WebIdentity {
    fn default() -> Self {
        Self {
            provider_urls: vec![format!("{}fed_cm/config", RauthyConfig::get().issuer)],
        }
    }
}
