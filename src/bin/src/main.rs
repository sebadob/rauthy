// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::cli_args::Args;
use clap::Parser;
use std::env;
use std::error::Error;

#[cfg(all(
    feature = "jemalloc",
    not(target_env = "msvc"),
    not(target_os = "openbsd")
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod cli_args;
mod dummy_data;
mod init_static_vars;
mod logging;
mod server;
mod tls;
mod utils;
mod version_migration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    match Args::parse() {
        Args::Serve(args) => {
            let local_test = env::var("LOCAL_TEST")
                .unwrap_or_else(|_| "false".to_string())
                .parse::<bool>()
                .expect("Cannot parse LOCAL_TEST as bool");

            if local_test {
                server::run("config-local-test.toml".to_string(), false).await?
            } else {
                #[cfg(debug_assertions)]
                let test_mode = args.test;
                #[cfg(not(debug_assertions))]
                let test_mode = false;
                server::run(args.config_file, test_mode).await?
            }
        }
        Args::GenerateConfig(args) => utils::gen_config::generate(args).await?,
        Args::GenerateEncKey(args) => utils::gen_enc_keys::generate(args).await?,
        Args::GenerateSecrets => utils::gen_secrets::generate().await?,
    }

    Ok(())
}
