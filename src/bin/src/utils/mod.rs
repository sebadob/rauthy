pub mod gen_config;
pub mod gen_enc_keys;
pub mod gen_secrets;
pub mod hash_password;
mod stdin;
pub mod validate_config;

type StdError = Box<dyn std::error::Error>;
