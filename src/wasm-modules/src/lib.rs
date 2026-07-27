// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

#[cfg(feature = "md")]
pub mod markdown;

#[cfg(debug_assertions)]
mod utils;

#[cfg(feature = "spow")]
use wasm_bindgen::prelude::*;

/// Calculates the Proof of Work for the given challenge
#[cfg(feature = "spow")]
#[wasm_bindgen]
pub fn pow_work_wasm(challenge: &str) -> Option<String> {
    #[cfg(debug_assertions)]
    utils::set_panic_hook();
    spow::wasm::pow_work(challenge).ok()
}
