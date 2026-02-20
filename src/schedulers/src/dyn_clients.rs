use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_data::database::DB;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_dyn::ClientDyn;
use rauthy_data::entity::failed_backchannel_logout::FailedBackchannelLogout;
use rauthy_data::entity::user_login_states::UserLoginState;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_service::oidc::logout;
use std::ops::Sub;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info};

/// Cleans up unused dynamically registered clients
pub async fn dyn_client_cleanup() {
    if !RauthyConfig::get().vars.dynamic_clients.enable {
        info!(
            "Dynamic client registration is not enabled - exiting dynamic_client_cleanup scheduler"
        );
        return;
    }
    if RauthyConfig::get().vars.dynamic_clients.reg_token.is_some() {
        info!("Dynamic client registration is private - exiting dynamic_client_cleanup scheduler");
        return;
    }

    let mut interval = tokio::time::interval(Duration::from_secs(
        (RauthyConfig::get().vars.dynamic_clients.cleanup_interval as u64).saturating_mul(60),
    ));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping dynamic_client_cleanup scheduler"
            );
            continue;
        }
        debug!("Running dynamic_client_cleanup scheduler");

        let cleanup_inactive_days = RauthyConfig::get()
            .vars
            .dynamic_clients
            .cleanup_inactive_days;

        let threshold_inactive = if cleanup_inactive_days > 0 {
            Utc::now()
                .sub(chrono::Duration::days(cleanup_inactive_days as i64))
                .timestamp()
        } else {
            0
        };

        let sql = "SELECT * FROM clients_dyn WHERE last_used IS NULL OR last_used < $1";

        let clients_res: Result<Vec<ClientDyn>, String> = if is_hiqlite() {
            DB::hql()
                .query_as(sql, params!(threshold_inactive))
                .await
                .map_err(|err| err.to_string())
        } else {
            DB::pg_query(sql, &[&threshold_inactive], 0)
                .await
                .map_err(|err| err.to_string())
        };
        let clients: Vec<ClientDyn> = match clients_res {
            Ok(c) => c,
            Err(err) => {
                error!(error = err);
                continue;
            }
        };

        let threshold = Utc::now().timestamp()
            - RauthyConfig::get().vars.dynamic_clients.cleanup_minutes as i64;
        let mut cleaned_up = 0;
        for client in clients {
            let should_delete = if client.last_used.is_some() {
                cleanup_inactive_days > 0
            } else {
                client.created < threshold
            };

            if should_delete {
                info!("Cleaning up unused dynamic client {}", client.id);
                match Client::find(client.id).await {
                    Ok(c) => {
                        if client.last_used.is_some() {
                            if let Err(err) = logout::execute_backchannel_logout_by_client(&c).await
                            {
                                error!(
                                    "Error during async backchannel logout after client delete: {:?}",
                                    err
                                );
                                let _ = UserLoginState::delete_all_by_cid(c.id.clone()).await;
                            }

                            if let Err(err) =
                                FailedBackchannelLogout::delete_all_by_client(c.id.clone()).await
                            {
                                error!("Error cleaning up FailedBackchannelLogouts: {:?}", err);
                            }
                        }

                        if let Err(err) = c.delete().await {
                            error!(?err, "deleting unused client");
                            continue;
                        }

                        cleaned_up += 1;
                    }
                    Err(err) => {
                        error!(
                            "Client does not exist for ClientDyn when it should. This should never happen.\
                        Please report this issue: {err:?}"
                        );
                        continue;
                    }
                }
            }
        }

        if cleaned_up > 0 {
            info!("Cleaned up {cleaned_up} unused dynamic clients");
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
