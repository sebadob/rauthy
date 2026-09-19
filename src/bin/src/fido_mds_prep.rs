//! Offline tool that regenerates the shipped FIDO MDS dataset.
//!
//! It downloads (or reads) the raw MDS blob, runs the exact transform the server uses, and writes
//! the serialized [`MdsDataset`] to the asset the server embeds at compile time. Run it via the
//! `just fido-mds-prep` recipe rather than directly.
//!
//! The asset is not checked into the repository: `just setup` and `just build` fetch it. Note that
//! `https` only authenticates the server we download from, it does not verify the blob itself.
//! Anything but `https` is therefore rejected, and a `--source` pointing at a local file is
//! trusted as far as whoever put the file there.

use clap::Parser;
use rauthy_data::fido_mds::MdsDataset;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

const DEFAULT_SOURCE: &str = "https://mds.fidoalliance.org/";
const DEFAULT_OUT: &str = "assets/fido_mds/dataset.bin";

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about = "Prepares the FIDO MDS dataset Rauthy embeds at compile time",
    long_about = None
)]
struct Args {
    /// The MDS blob to transform: an `https://` URL, or a path to a local file.
    #[clap(short, long, default_value = DEFAULT_SOURCE)]
    source: String,
    /// Where to write the prepared dataset.
    #[clap(short, long, default_value = DEFAULT_OUT)]
    out: PathBuf,
}

/// The metadata service rate-limits, and answers a throttled request with `200 OK` and a plain
/// text body rather than a status code, so the response has to be sniffed rather than trusted.
/// A release build fetches the dataset, so a transient throttle must not fail it outright.
const DOWNLOAD_ATTEMPTS: u32 = 5;

async fn download(source: &str) -> Result<String, Box<dyn Error>> {
    // reqwest follows redirects by default, and the alliance may redirect. Downgrading to plain
    // `http` on a hop would give up exactly the transport protection the caller checked for, so
    // every hop has to stay on `https` too.
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.url().scheme() == "https" {
                attempt.follow()
            } else {
                attempt.error("refusing to follow a redirect off https")
            }
        }))
        .build()?;

    let mut last_err = String::new();
    for attempt in 1..=DOWNLOAD_ATTEMPTS {
        if attempt > 1 {
            let backoff = Duration::from_secs(2u64.pow(attempt - 1));
            println!("Retrying in {}s: {last_err}", backoff.as_secs());
            tokio::time::sleep(backoff).await;
        }

        let res = client.get(source).send().await?;
        let status = res.status();
        let body = res.text().await?;

        // a blob is `header.payload.signature`, so anything without two dots is not one
        if status.is_success() && body.trim().split('.').count() == 3 {
            return Ok(body);
        }
        last_err = format!(
            "the metadata service answered {status} with {:?}",
            body.trim().chars().take(120).collect::<String>()
        );
    }

    Err(format!("giving up after {DOWNLOAD_ATTEMPTS} attempts, {last_err}").into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let jwt = if args.source.starts_with("http://") {
        return Err("refusing to download the FIDO MDS blob over plain http".into());
    } else if args.source.starts_with("https://") {
        println!("Downloading FIDO MDS blob from {}", args.source);
        download(&args.source).await?
    } else if args.source.contains("://") {
        return Err(format!(
            "unsupported FIDO MDS source scheme in {:?}, expected `https://` or a file path",
            args.source
        )
        .into());
    } else {
        println!("Reading FIDO MDS blob from {}", args.source);
        std::fs::read_to_string(&args.source)?
    };

    let dataset = MdsDataset::from_str(jwt.trim()).map_err(|err| err.to_string())?;
    let next_update = chrono::DateTime::from_timestamp(dataset.next_update_ts, 0)
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| dataset.next_update_ts.to_string());
    println!(
        "Prepared blob no. {}, nextUpdate {}: {} entries, {} distinct root certs",
        dataset.blob_no,
        next_update,
        dataset.entries.len(),
        dataset.certs.len(),
    );

    let bytes = dataset.serialize().map_err(|err| err.to_string())?;
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&args.out, &bytes)?;
    println!("Wrote {} bytes to {}", bytes.len(), args.out.display());

    Ok(())
}
