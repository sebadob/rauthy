use crate::language::Language;

pub mod account;
pub mod authorize;
pub mod device;
pub mod email_change_info_new;
pub mod email_change_info_old;
pub mod email_confirm_change;
pub mod email_confirm_change_html;
pub mod email_password_new;
pub mod email_reset;
pub mod email_reset_info;
pub mod error;
pub mod index;
pub mod logout;
pub mod password_policy;
pub mod password_reset;
pub mod register;

pub trait SsrJson {
    fn build(lang: &Language) -> Self;
    fn as_json(&self) -> String;
}
