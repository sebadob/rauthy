//! Makes the prepared FIDO MDS dataset available to `include_bytes!`.
//!
//! `assets/fido_mds/dataset.bin` is produced by `just fido-mds-prep` and is not checked in, so
//! there is exactly one case where it is legitimately absent: a tree that has not fetched it yet.
//! That case has to keep compiling, because `fido-mds-prep` links this crate, so failing the build
//! on a missing asset would mean the tool that creates the asset cannot be built. The Code Style
//! workflow relies on the same allowance, since CI never runs `just setup`.
//!
//! The allowance is bounded to exactly that case:
//!
//! - missing or empty, debug profile: an empty placeholder, with a warning. Seeding it is a no-op.
//! - missing or empty, release profile: a hard error. A release binary that silently ships no
//!   dataset is
//!   the failure this is guarding against. To bootstrap from a fresh clone, run
//!   `just fido-mds-prep` first, which builds the tool in the debug profile.
//! - unreadable for any other reason (permissions, a directory, a partial read): a hard error in
//!   every profile. Only `NotFound` means "not fetched yet"; everything else is a broken tree.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{env, fs};

const ASSET: &str = "assets/fido_mds/dataset.bin";

fn main() {
    let asset = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(ASSET);
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("fido_mds_dataset.bin");

    println!("cargo::rerun-if-changed={}", asset.display());

    let bytes = match fs::read(&asset) {
        // an existing but empty file is a truncated asset, not a dataset, so it takes the same
        // route as an absent one rather than being embedded as a valid empty dataset
        Ok(bytes) if !bytes.is_empty() => bytes,
        Ok(_) => missing("is empty"),
        Err(err) if err.kind() == ErrorKind::NotFound => missing("is missing"),
        Err(err) => panic!("cannot read {ASSET}: {err}"),
    };

    fs::write(&out, bytes).expect("writing the FIDO MDS dataset into OUT_DIR");
}

/// The one bounded allowance: a tree that has not fetched the dataset yet still builds, in the
/// debug profile only, so that `fido-mds-prep` can be built to create it.
fn missing(why: &str) -> Vec<u8> {
    assert!(
        env::var("PROFILE").unwrap_or_default() != "release",
        "{ASSET} {why} and this is a release build. Run `just fido-mds-prep` to fetch it, then \
         build again."
    );
    println!(
        "cargo::warning={ASSET} {why}, building without a FIDO MDS dataset. Run \
         `just fido-mds-prep` to fetch it."
    );
    Vec::new()
}
