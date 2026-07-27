use crate::cli_args::ArgsGenKey;
use crate::utils::StdError;
use chrono::Utc;
use cryptr::EncKeys;
use std::fmt::Write;

pub async fn generate(args: ArgsGenKey) -> Result<(), StdError> {
    let kid = if let Some(kid) = args.with_key_id {
        kid
    } else {
        Utc::now().date_naive().to_string()
    };
    if !EncKeys::re_key_id().is_match(&kid) {
        return Err("Invalid Key ID. Must match: ^[a-zA-Z0-9:_-]{{2,20}}$".into());
    }

    let keys = EncKeys::generate_with_id(kid)?;

    let mut out = String::with_capacity(128);

    writeln!(out, "\n[encryption]")?;
    writeln!(out, "keys = [")?;
    for key in keys.keys_as_b64_vec() {
        writeln!(out, "    '{}',", key)?;
    }
    writeln!(out, "]")?;
    writeln!(out, "key_active = '{}'", keys.enc_key_active)?;

    println!("{out}");

    Ok(())
}
