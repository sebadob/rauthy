use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Args {
    /// Start the Rauthy server
    Serve(ArgsServer),

    /// Generate a config file to get you started.
    GenerateConfig(ArgsGenConfig),

    /// Generate a new secure, random encryption key
    GenerateEncKey(ArgsGenKey),

    /// Generate Raft Cluster secrets
    GenerateSecrets,
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsServer {
    /// Provide a custom path to the config file
    #[clap(short, long, default_value = "./config.toml")]
    pub config_file: String,

    /// Integration Test Mode
    #[cfg(debug_assertions)]
    #[clap(short, long, default_value_t = false)]
    pub test: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsGenConfig {
    /// Where the config should be written to. This file must not exist yet.
    #[clap(short, long)]
    pub output_file: String,

    /// If set, the `output_file` will be overwritten, if it exists.
    #[clap(long, default_value_t = false)]
    pub overwrite: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsGenKey {
    /// Provide if you want a specific Key ID - must match: [a-zA-Z0-9:_-]{2,20}
    #[clap(short = 'k', long)]
    pub with_key_id: Option<String>,
}
