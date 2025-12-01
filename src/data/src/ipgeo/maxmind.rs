use crate::ipgeo::LookupResponse;
use crate::rauthy_config::RauthyConfig;
use arc_swap::ArcSwap;
use flate2::bufread;
use futures::stream::StreamExt;
use maxminddb::geoip2;
use rauthy_common::http_client;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::net::IpAddr;
use std::sync::{Arc, OnceLock};
use tar::Archive;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::{fs, pin, task};
use tracing::{debug, error, info};
// static LINK_CITY: &str =
//     "https://download.maxmind.com/geoip/databases/GeoLite2-City/download?suffix=tar.gz";
// static LINK_COUNTRY: &str =
//     "https://download.maxmind.com/geoip/databases/GeoLite2-Country/download?suffix=tar.gz";

static BASE_DIR: OnceLock<String> = OnceLock::new();
static ACCOUNT_ID: OnceLock<String> = OnceLock::new();
static LICENSE_KEY: OnceLock<String> = OnceLock::new();
static GEO_DB: OnceLock<ArcSwap<maxminddb::Reader<Vec<u8>>>> = OnceLock::new();

pub async fn update_db() -> Result<(), ErrorResponse> {
    info!("Updating GeoLite-Country DB");

    let base_dir = BASE_DIR.get().unwrap();
    let db_file = format!("{base_dir}/db.mmdb");
    let etag_path = format!("{base_dir}/etag");
    let username = ACCOUNT_ID.get().unwrap();
    let password = LICENSE_KEY.get().unwrap();

    let db_type = RauthyConfig::get().vars.geo.maxmind_db_type.as_ref();
    let link =
        format!("https://download.maxmind.com/geoip/databases/{db_type}/download?suffix=tar.gz");

    // before downloading, check the version
    if fs::try_exists(&db_file).await? {
        debug!("GeoLite DB already exists");
        let etag = fs::read_to_string(&etag_path).await?;
        debug!("Current GeoLite DB ETAG: {etag}");

        // TODO should be allow skipping this step if there is a general network issue?
        //  maybe also save the timestamp for the etag to be able to log warnings for too old DB?
        let res = http_client()
            .head(&link)
            .basic_auth(username, Some(password))
            .send()
            .await?;
        debug!("GeoLite version check res: {res:?}");
        if res.status().is_success() {
            let etag_header = res
                .headers()
                .get("etag")
                .map(|v| v.to_str().unwrap_or_default())
                .unwrap_or_default();
            if etag == etag_header {
                info!("Maxmind DB is up to date");

                if GEO_DB.get().is_none() {
                    match build_reader(db_file.clone()).await {
                        Ok(reader) => {
                            GEO_DB.set(ArcSwap::new(Arc::new(reader))).unwrap();
                            return Ok(());
                        }
                        Err(err) => {
                            error!(?err, "Error opening Maxmind DB - starting new download");
                        }
                    }
                } else {
                    return Ok(());
                }
            }
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                format!("Error checking Maxmind DB version: HTTP {}", res.status()),
            ));
        }
    }

    info!("No existing GeoLite DB found - starting download");

    // either no DB exists or version is outdated -> download new
    let res = http_client()
        .get(link)
        .basic_auth(username, Some(password))
        .send()
        .await?;

    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            format!("Error download Maxmind DB: {body}"),
        ));
    }

    let etag = res
        .headers()
        .get("etag")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default()
        .to_string();
    debug!("New GeoLite ETAG: {etag}");

    let stream = res.bytes_stream();
    pin!(stream);

    // we can safely remove it upfront - the reader loads it into memory
    let path_archive = format!("{base_dir}/db.tar.gz");
    debug!("Writing archive to: {path_archive}");
    let _ = fs::remove_file(&path_archive).await;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path_archive)
        .await?;

    while let Some(Ok(item)) = stream.next().await {
        file.write_all(item.as_ref()).await?;
    }
    // TODO we maybe could add a sha256 check
    file.sync_all().await?;
    drop(file);
    debug!("GeoLite archive written");

    // decompress
    let path_archive_clone = path_archive.clone();
    let db_file_clone = db_file.clone();
    task::spawn_blocking(move || decompress(&path_archive_clone, base_dir, &db_file_clone))
        .await
        .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))??;

    fs::write(etag_path, etag).await?;
    fs::remove_file(path_archive).await?;

    let reader = build_reader(db_file).await?;
    if let Some(db) = GEO_DB.get() {
        db.store(Arc::new(reader));
    } else {
        GEO_DB.set(ArcSwap::new(Arc::new(reader))).unwrap();
    }

    Ok(())
}

pub(super) async fn init(
    data_dir: &str,
    account_id: String,
    license_key: String,
) -> Result<(), ErrorResponse> {
    info!("Initializing Maxmind GeoLite DB");

    ACCOUNT_ID.set(account_id).unwrap();
    LICENSE_KEY.set(license_key).unwrap();

    let base_dir = format!("{data_dir}/ipgeo");
    fs::create_dir_all(&base_dir).await?;
    BASE_DIR.set(base_dir).unwrap();

    update_db().await?;

    Ok(())
}

/// CAUTION: Blocking!
fn decompress(input_file: &str, output_dir: &str, db_file: &str) -> Result<(), ErrorResponse> {
    debug!(
        "Decompressing GeoLite Archive from {input_file} into {output_dir} with DB target {db_file}"
    );

    let input = std::io::BufReader::new(std::fs::File::open(input_file)?);
    let mut archive = Archive::new(bufread::MultiGzDecoder::new(input));

    let mut is_first = true;
    let mut prefix: Option<String> = None;
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        debug!("Entry in archive: {path:?}");
        let path_str = if let Some(prefix) = &prefix {
            match path.strip_prefix(prefix) {
                Ok(s) => s,
                Err(_) => path.as_ref(),
            }
        } else {
            path.as_ref()
        }
        .to_str()
        .unwrap_or_default();

        // for maxmind, we will always have a single directory
        if is_first {
            if path_str.ends_with("/")
                && let Some(stripped) = path_str.strip_suffix("/")
            {
                prefix = Some(stripped.to_string());
                is_first = false;
                continue;
            }
            is_first = false;
        }

        if path_str.ends_with(".mmdb") {
            entry.unpack(db_file)?;
        } else {
            entry.unpack(format!("{output_dir}/{path_str}"))?;
        };
    }

    Ok(())
}

#[inline]
pub(super) fn is_configured() -> bool {
    GEO_DB.get().is_some()
}

pub(super) fn get_location(ip: IpAddr) -> Result<Option<LookupResponse>, ErrorResponse> {
    let Some(rdr) = GEO_DB.get() else {
        return Ok(None);
    };

    let rdr = rdr.load();
    let res = rdr
        .lookup(ip)
        .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))?;
    if !res.has_data() {
        return Ok(None);
    }
    let city: Option<geoip2::City> = res
        .decode()
        .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))?;

    if let Some(city) = city {
        let Some(alpha_2_code) = city.country.iso_code else {
            return Ok(None);
        };

        return Ok(Some(LookupResponse {
            alpha_2_code: alpha_2_code.to_string(),
            country: city.country.names.english.unwrap_or_default().to_string(),
            city: city.city.names.english.map(String::from),
        }));
    }

    Ok(None)
}

#[inline]
async fn build_reader(db_file: String) -> Result<maxminddb::Reader<Vec<u8>>, ErrorResponse> {
    task::spawn_blocking(move || {
        maxminddb::Reader::open_readfile(&db_file)
            .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))
    })
    .await
    .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))?
}
