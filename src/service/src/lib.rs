// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

pub mod client;
pub mod encryption;
pub mod login_delay;
pub mod oidc;
pub mod password_reset;
mod scim;
pub mod suspicious_request_block;
pub mod token_set;
