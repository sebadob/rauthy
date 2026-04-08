pub mod gen_config;
pub mod gen_enc_keys;
pub mod gen_secrets;
mod stdin;

type StdError = Box<dyn std::error::Error>;
