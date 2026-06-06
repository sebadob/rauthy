use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Args {
    /// Start the Rauthy server
    Serve(ArgsServer),

    /// Read or purge generated bootstrap secrets from the local encrypted container.
    Bootstrap(ArgsBootstrap),

    /// Generate a config file to get you started.
    GenerateConfig(ArgsGenConfig),

    /// Validate a Rauthy config file.
    ValidateConfig(ArgsValidateConfig),

    /// Generate a new secure, random encryption key
    GenerateEncKey(ArgsGenKey),

    /// Generate Raft Cluster secrets
    GenerateSecrets,

    HashPassword(ArgsPasswordHash),
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsBootstrap {
    #[command(subcommand)]
    pub command: ArgsBootstrapCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ArgsBootstrapCommand {
    /// Decrypt and print generated bootstrap secrets.
    Get(ArgsBootstrapGet),

    /// Delete the generated bootstrap secret container.
    Purge(ArgsBootstrapPurge),
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsBootstrapGet {
    /// Path to the encrypted generated-secret container.
    #[clap(short, long, default_value = "data/bootstrap.secrets.enc")]
    pub file: String,

    /// Output format.
    #[clap(long, value_enum, default_value_t = BootstrapOutputFormat::Env)]
    pub format: BootstrapOutputFormat,

    /// Filter by entry kind, for example `client` or `user`.
    #[clap(long)]
    pub kind: Option<String>,

    /// Filter by entry id, for example the client id or user email.
    #[clap(long)]
    pub id: Option<String>,

    /// Filter by field, for example `secret` or `password`.
    #[clap(long)]
    pub field: Option<String>,

    /// Active encryption key id. Falls back to ENC_KEY_ACTIVE.
    #[clap(long, env = "ENC_KEY_ACTIVE")]
    pub enc_key_active: Option<String>,

    /// Newline-separated encryption keys. Falls back to ENC_KEYS.
    #[clap(long, env = "ENC_KEYS")]
    pub enc_keys: Option<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsBootstrapPurge {
    /// Path to the encrypted generated-secret container.
    #[clap(short, long, default_value = "data/bootstrap.secrets.enc")]
    pub file: String,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BootstrapOutputFormat {
    Raw,
    Json,
    Env,
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
pub struct ArgsValidateConfig {
    /// The path to the config file.
    #[clap(short, long)]
    pub path: String,
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsGenKey {
    /// Provide if you want a specific Key ID - must match: [a-zA-Z0-9:_-]{2,20}
    #[clap(short = 'k', long)]
    pub with_key_id: Option<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct ArgsPasswordHash {
    /// The amount of memory in bytes used during the key derivation process. MUST NOT be below 32768.
    #[clap(short, long, default_value = "131072")]
    pub m_cost: Option<u32>,

    /// The number of iterations performed during the key derivation process.
    #[clap(short, long, default_value = "4")]
    pub t_cost: Option<u32>,

    /// The number of parallel threads used during computation.
    #[clap(short, long, default_value = "8")]
    pub p_cost: Option<u32>,
}
