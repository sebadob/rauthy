use rauthy_data::database::DB;
use rauthy_data::entity::email_jobs::EmailJob;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info};

pub async fn orphaned_email_jobs() {
    let mut interval = tokio::time::interval(Duration::from_secs(
        RauthyConfig::get()
            .vars
            .email
            .jobs
            .scheduler_interval_seconds as u64,
    ));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping orphaned_email_jobs scheduler"
            );
            continue;
        }
        debug!("Running orphaned_email_jobs scheduler");

        if let Err(err) = execute().await {
            error!("Error during orphaned_email_jobs: {:?}", err);
        }

        time::sleep(Duration::from_secs(3)).await;
    }
}

#[inline]
async fn execute() -> Result<(), ErrorResponse> {
    let jobs = EmailJob::find_orphaned_or_schedule().await?;
    if jobs.is_empty() {
        return Ok(());
    }

    info!(
        "Found {} orphaned or scheduled EmailJobs - (re)starting them now",
        jobs.len()
    );

    for job in jobs {
        job.spawn_task();

        // no need to spike resources and spawn multiple at the same time
        time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
