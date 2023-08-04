use crate::language::Language;

pub mod account;
pub mod authorize;
pub mod password_policy;

pub trait SsrJson {
    fn build(lang: &Language) -> Self;
    fn as_json(&self) -> String;
}
