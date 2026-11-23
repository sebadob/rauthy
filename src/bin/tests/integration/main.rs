//! All integration tests live in this single test binary. One module per tested area, shared
//! fixtures and assertion helpers in `common`. A new area adds a file and one `mod` line.

#[allow(dead_code)]
mod common;

mod api_keys;
mod auth_flow;
mod backchannel_logout;
mod client_binding;
mod clients;
mod clients_dyn;
mod cust_attrs;
mod generic;
mod group_admin;
mod groups;
mod logout_redirect;
mod mfa_admin_assurance;
mod otp_email_secret;
mod password_policy;
mod provider_callback;
mod provider_claim_mappings;
mod provider_hide_login;
mod pwd_reset;
mod refresh_session_logout;
mod resource_indicators;
mod roles;
mod scopes;
mod token_exchange;
mod users;
