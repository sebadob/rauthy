//! Makes the prepared FIDO MDS dataset available to `include_bytes!` without requiring it to be
//! present.
//!
//! `assets/fido_mds/dataset.bin` is produced by `just fido-mds-prep` and is not checked in. If it
//! is missing, an empty placeholder is emitted instead of failing the build, for two reasons:
//! `fido-mds-prep` itself links this crate, so a hard failure would be a bootstrap cycle on a
//! fresh checkout, and CI only runs the style checks without fetching the dataset. Seeding an
//! empty dataset is a no-op that logs a warning at startup.

use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let asset = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fido_mds/dataset.bin");
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("fido_mds_dataset.bin");

    println!("cargo::rerun-if-changed={}", asset.display());

    let bytes = fs::read(&asset).unwrap_or_else(|_| {
        println!(
            "cargo::warning=assets/fido_mds/dataset.bin is missing, building without a FIDO MDS \
             dataset. Run `just fido-mds-prep` to fetch it."
        );
        Vec::new()
    });
    fs::write(&out, bytes).expect("writing the FIDO MDS dataset into OUT_DIR");
}
