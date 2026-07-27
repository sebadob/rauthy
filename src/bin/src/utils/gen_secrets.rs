use crate::utils::StdError;
use cryptr::utils::secure_random_alnum;
use std::fmt::Write;

pub async fn generate() -> Result<(), StdError> {
    let mut out = String::with_capacity(128);

    writeln!(out, "\n[cluster]")?;
    writeln!(out, "secret_raft = '{}'", secure_random_alnum(48))?;
    writeln!(out, "secret_api = '{}'", secure_random_alnum(48))?;

    println!("{out}");

    Ok(())
}
