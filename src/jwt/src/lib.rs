// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use rauthy_error::ErrorResponse;
use rauthy_models::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};

pub mod claims;
pub mod token;

pub async fn test_jwk_compat() -> Result<(), ErrorResponse> {
    let jwk = JwkKeyPair::find_latest(JwkKeyPairAlg::RS256).await?;

    let msg = b"Some Message";
    jwk.sign(msg.as_slice())?;

    todo!()
}
