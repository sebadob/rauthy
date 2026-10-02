use chrono::{DateTime, Datelike, TimeZone, Utc};
use rauthy_data::database::DB;
use rauthy_data::email::notification::send_email_notification;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::sponsor_reminder::SponsorReminder;
use rauthy_data::entity::users::User;
use rauthy_data::events::notifier::{NOTIFIER_MATRIX, NOTIFIER_SLACK};
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use rauthy_notify::{Notification, NotificationLevel, Notify};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

/// We don't want to annoy the user, just ask for a sponsoring once a year like KDE does it.
/// We want to do it only between 6.12. and 28.12. To only do it once between restarts, let
/// only the leader do it, and then insert a record into the DB.
pub async fn run_sponsor_reminder() {
    time::sleep(Duration::from_secs(1)).await;
    if RauthyConfig::get().vars.sponsor.email_reminder_disable {
        info!("Project sponsoring Notifications disabled. Exiting scheduler.");
        return;
    }
    debug!("Project sponsoring Notifications enabled.");

    // Do a short sleep before starting, just in case we are inside the time window.
    time::sleep(Duration::from_secs(300)).await;

    loop {
        let now = Utc::now();
        let (from, until) = sponsor_window(now.year());

        // sleep until the yearly sponsor window is reached
        if now < from || now > until {
            let target = if now < from {
                from
            } else {
                sponsor_window(now.year() + 1).0
            };
            debug!(
                ?target,
                "Project sponsoring reminder: sleeping until the sponsor window opens"
            );
            time::sleep(target.signed_duration_since(now).to_std().unwrap()).await;
            continue;
        }

        // if it was already sent this year (possibly by another leader), sleep until next year
        if let Some(ts) = SponsorReminder::find().await
            && let Some(last) = DateTime::from_timestamp(ts, 0)
            && last.year() == now.year()
        {
            let target = sponsor_window(now.year() + 1).0;
            debug!(
                ?target,
                "Project sponsoring reminder: already sent this year, sleeping until next year"
            );
            time::sleep(target.signed_duration_since(now).to_std().unwrap()).await;
            continue;
        }

        // only the leader sends, so we don't send it multiple times in HA deployments. If there
        // is no leader at all (election ongoing), just sleep a short time and check again.
        if DB::hql().is_leader_cache().await {
            if let Err(err) = execute(now).await {
                error!(?err, "Executing the Project sponsoring reminder failed");
            }
        } else {
            debug!("Project sponsoring reminder: not the leader yet - checking again shortly");
        }

        time::sleep(Duration::from_secs(600)).await;
    }
}

/// Returns the yearly sponsor reminder window (6.12. 12:00 UTC - 28.12. 12:00 UTC)
fn sponsor_window(year: i32) -> (DateTime<Utc>, DateTime<Utc>) {
    let from = Utc.with_ymd_and_hms(year, 12, 6, 12, 0, 0).unwrap();
    let until = Utc.with_ymd_and_hms(year, 12, 28, 12, 0, 0).unwrap();
    (from, until)
}

async fn execute(now: DateTime<Utc>) -> Result<(), ErrorResponse> {
    // one notification drives the themed e-mail as well as the slack and matrix notifications
    let notification = Notification {
        level: NotificationLevel::Info,
        head: "Support the Rauthy project".to_string(),
        row_1: "If you find Rauthy useful, please consider supporting its continued development \
               by becoming a sponsor or making a donation."
            .to_string(),
        row_2: Some(
            "You can do so via GitHub Sponsors: https://github.com/sponsors/sebadob".to_string(),
        ),
    };

    // the e-mail recipients are the `rauthy_admin_email` from the config (if set) and the
    // contacts of the "rauthy" client, deduped
    let mut recipients = find_email_recipients().await;

    // also send it to the slack and matrix notification channels from the `events` section in
    // the config, if they are configured
    let channels_configured = send_channel_notifications(&notification).await;

    // if neither side produced a target, fall back to the 5 oldest accounts with the full
    // `rauthy_admin` role, so we get at least one message out there
    if recipients.is_empty() && !channels_configured {
        recipients = find_fallback_recipients().await?;
    }

    if !recipients.is_empty() {
        // send the themed e-mail to all recipients, personalized with a greeting if we know
        // their first name
        for recipient in &recipients {
            let mut personal = notification.clone();
            if let Some(name) = &recipient.name {
                personal.row_1 = format!(
                    "Dear {name}, if you find Rauthy useful, please consider supporting its \
                     continued development by becoming a sponsor or making a donation."
                );
            }

            send_email_notification(
                recipient
                    .name
                    .clone()
                    .unwrap_or_else(|| "Rauthy Admin".to_string()),
                recipient.email.clone(),
                &RauthyConfig::get().tx_email,
                &personal,
            )
            .await;
        }

        info!(
            "Project sponsoring reminder sent to {} recipient(s)",
            recipients.len()
        );
    } else {
        if channels_configured {
            info!("Project sponsoring reminder sent via the configured notification channel(s)");
        } else {
            warn!("No recipients found for the Project sponsoring reminder - skipping");
            return Ok(());
        }
    }

    // mark it as sent, so we only do this once a year
    SponsorReminder::upsert(now.timestamp()).await?;

    Ok(())
}

/// A recipient of the sponsor reminder e-mail. `name` is the user's first name, if we could
/// look one up in the database - otherwise the generic message is sent.
struct Recipient {
    name: Option<String>,
    email: String,
}

/// Returns the deduped recipients from the `rauthy_admin_email` config value and the contacts
/// of the "rauthy" client, together with their first names (if we can look them up in the
/// database).
async fn find_email_recipients() -> Vec<Recipient> {
    let mut emails = HashSet::new();
    if let Some(email) = RauthyConfig::get().vars.email.rauthy_admin_email.clone()
        && !email.is_empty()
    {
        emails.insert(email);
    }

    match Client::find("rauthy".to_string()).await {
        Ok(client) => emails.extend(client.get_contacts().unwrap_or_default()),
        Err(err) => error!(
            ?err,
            "Finding the 'rauthy' client for the Project sponsoring reminder"
        ),
    }

    let mut recipients = Vec::with_capacity(emails.len());
    for email in emails {
        recipients.push(Recipient {
            name: find_first_name(&email).await,
            email,
        });
    }

    recipients
}

/// Looks up the user with the given e-mail address and returns their first name, if they
/// exist and have one set.
async fn find_first_name(email: &str) -> Option<String> {
    match User::find_by_email(email.to_string()).await {
        Ok(user) => first_name(&user),
        Err(_) => None,
    }
}

/// Returns the user's first name, if they have one set.
fn first_name(user: &User) -> Option<String> {
    let name = user.given_name.trim();
    (!name.is_empty()).then_some(name.to_string())
}

/// Sends the notification to the slack and matrix notifiers (built once during startup, if they
/// are configured). Returns whether at least one of them accepted the notification.
async fn send_channel_notifications(notification: &Notification) -> bool {
    let mut channels_configured = false;

    if let Some((_, notifier)) = NOTIFIER_SLACK.get() {
        match notifier.notify(notification).await {
            Ok(_) => {
                channels_configured = true;
            }
            Err(err) => {
                error!(?err, "Sending the Project sponsoring reminder via Slack");
            }
        }
    }

    if let Some((_, notifier)) = NOTIFIER_MATRIX.get() {
        match notifier.notify(notification).await {
            Ok(_) => {
                channels_configured = true;
            }
            Err(err) => {
                error!(?err, "Sending the Project sponsoring reminder via Matrix");
            }
        }
    }

    channels_configured
}

/// Fallback recipients when neither e-mail recipients nor notification channels are configured:
/// the 5 oldest accounts with the full `rauthy_admin` role. Group admins
/// (`rauthy_admin:<group>`) are skipped, since they might be customer accounts.
async fn find_fallback_recipients() -> Result<Vec<Recipient>, ErrorResponse> {
    // `find_all` already returns the users ordered by `created_at ASC`
    let users = User::find_all().await?;

    Ok(users
        .iter()
        .filter(|u| u.is_admin())
        .take(5)
        .map(|u| Recipient {
            name: first_name(u),
            email: u.email.clone(),
        })
        .collect())
}
