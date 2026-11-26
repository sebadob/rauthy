use crate::{
    database::DB,
    entity::{
        db_version::DbVersion,
        one_time_password::{OneTimePassword, OtpSecret},
    },
};
use hiqlite::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use semver::Version;
use tracing::{debug, warn};

pub async fn apply_temp_migrations(
    previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    let app_version = DbVersion::app_version();
    if let Some(prev) = previous_db_version
        && prev.major == app_version.major
        && prev.minor == app_version.minor
    {
        // This version was already started before - nothing to do.
        // This function never changes between patch releases.
        return Ok(());
    }

    // We changed a few cached entities which would lead to decode errors from old data.
    DB::cache().clear_cache_all().await?;
    rotate_empty_otp_secrets().await?;

    Ok(())
}

async fn rotate_empty_otp_secrets() -> Result<(), ErrorResponse> {
    debug!("Executing rotate_empty_otp_secrets");
    for otp in find_otps_with_empty_secret().await? {
        warn!(
            otp.id = otp.id,
            otp.user_id = otp.user_id,
            otp.kind = otp.kind.as_str(),
            "Rotated an empty OTP secret - the user needs to request a new code"
        );
        rotate_otp_secret(otp).await?;
    }
    Ok(())
}

async fn find_otps_with_empty_secret() -> Result<Vec<OneTimePassword>, ErrorResponse> {
    let sql = if is_hiqlite() {
        "SELECT * FROM one_time_password WHERE length(secret) = 0"
    } else {
        "SELECT * FROM one_time_password WHERE octet_length(secret) = 0"
    };
    let res = if is_hiqlite() {
        DB::hql().query_as(sql, params!()).await?
    } else {
        DB::pg_query(sql, &[], 0).await?
    };

    Ok(res)
}

async fn rotate_otp_secret(otp: OneTimePassword) -> Result<(), ErrorResponse> {
    let secret = OtpSecret::generate()?;

    let sql = "UPDATE one_time_password SET secret = $1 WHERE id = $2";
    if is_hiqlite() {
        DB::hql().execute(sql, params!(secret.0, otp.id)).await?;
    } else {
        DB::pg_execute(sql, &[&secret.0, &otp.id]).await?;
    }

    Ok(())
}
