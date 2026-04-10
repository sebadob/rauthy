pub mod gen_config;
pub mod gen_enc_keys;
pub mod gen_secrets;
pub mod hash_password;
mod stdin;

type StdError = Box<dyn std::error::Error>;
