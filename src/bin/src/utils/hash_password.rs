use crate::cli_args::ArgsPasswordHash;
use crate::utils::StdError;
use crate::utils::stdin::PromptPassword;
use argon2_rust::params::{Memory, TagLen};
use argon2_rust::{Algorithm, Argon2, Params, Version};
use colored::Colorize;
use tokio::time::Instant;
use zeroize::Zeroize;

pub async fn hash(args: ArgsPasswordHash) -> Result<(), StdError> {
    println!("Argon2ID password hasher.");

    let m_cost = args.m_cost.unwrap_or(131_072);
    let t_cost = args.t_cost.unwrap_or(4);
    let p_cost = args.p_cost.unwrap_or(8);

    if m_cost < 32768 {
        return Err("m_cost must never be below 32768 to be secure.".into());
    }

    println!("\nInput your password:");

    let msg = r#"

To be as secure as possible, Rauthy comes with a default
password policy you must match. At least:
- 14 characters long
- 1 lowercase char
- 1 uppercase char
- 1 digit
"#
    .to_string();

    let policy = PromptPassword::default();
    let mut plain;
    loop {
        plain = policy.prompt_validated(&msg).await?;

        let plain_confirm = policy
            .prompt("Please enter the same password again for confirmation".to_string())
            .await?;
        if plain == plain_confirm {
            break;
        }
        eprintln!(
            "{}",
            "The two passwords do not match, please try again.".red()
        );
    }

    println!("Hashing password ...");

    let params = Params::builder()
        .memory(Memory::kib(m_cost as u64))
        .passes(t_cost)
        .lanes(p_cost)
        .tag_len(TagLen::bytes(32))
        .build()?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let start = Instant::now();
    let hashed = argon2.hash_password_with_random_salt(plain.as_bytes())?;

    let elapsed = start.elapsed().as_millis();
    if elapsed < 500 {
        println!(
            r#"
WARNING: The time taken to hash the password was {elapsed} ms, which is below the
recommended minimum of 500 ms. If this was done on your final deployment hardware,
consider bumping up the m_cost or t_cost.
"#,
        );
    }

    plain.zeroize();

    println!("Your password as Argon2ID hash:\n\n{hashed}");

    Ok(())
}
