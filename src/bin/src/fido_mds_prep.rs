//! Offline tool that regenerates the shipped FIDO MDS dataset.
//!
//! It downloads (or reads) the raw MDS blob, runs the exact transform the server uses, and writes
//! the serialized [`PreparedDataset`] to the asset the server embeds. Run it via the `just`
//! recipe rather than directly:
//!
//! ```text
//! cargo run --bin fido-mds-prep -- --out assets/fido_mds/dataset.bin
//! ```

use rauthy_data::fido_mds::PreparedDataset;
use std::error::Error;
use std::path::PathBuf;

const DEFAULT_SOURCE: &str = "https://mds.fidoalliance.org/";
const DEFAULT_OUT: &str = "assets/fido_mds/dataset.bin";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut source = DEFAULT_SOURCE.to_string();
    let mut out = PathBuf::from(DEFAULT_OUT);

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source" => {
                source = args.next().ok_or("--source needs a value")?;
            }
            "--out" => {
                out = PathBuf::from(args.next().ok_or("--out needs a value")?);
            }
            "-h" | "--help" => {
                println!(
                    "fido-mds-prep [--source <url|file>] [--out <path>]\n  \
                     --source  MDS blob URL or local file (default: {DEFAULT_SOURCE})\n  \
                     --out     output path (default: {DEFAULT_OUT})"
                );
                return Ok(());
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let jwt = if source.starts_with("http://") || source.starts_with("https://") {
        println!("Downloading FIDO MDS blob from {source}");
        reqwest::Client::new()
            .get(&source)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?
    } else {
        println!("Reading FIDO MDS blob from {source}");
        std::fs::read_to_string(&source)?
    };

    let dataset = PreparedDataset::from_jwt(jwt.trim()).map_err(|err| err.to_string())?;
    println!(
        "Prepared blob no. {}, nextUpdate {}: {} entries, {} distinct root certs",
        dataset.blob_no,
        dataset.next_update,
        dataset.entries.len(),
        dataset.certs.len(),
    );

    let bytes = dataset.serialize().map_err(|err| err.to_string())?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&out, &bytes)?;
    println!("Wrote {} bytes to {}", bytes.len(), out.display());

    Ok(())
}
