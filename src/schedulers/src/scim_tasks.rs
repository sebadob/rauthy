use actix_web::web;
use rand::Rng;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients_scim::ClientScim;
use rauthy_models::entity::failed_scim_tasks::{FailedScimTask, ScimAction};
use rauthy_models::entity::groups::Group;
use rauthy_models::entity::users::User;
use rauthy_models::events::event::Event;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

pub async fn scim_task_retry(data: web::Data<AppState>) {
    let retry_count = env::var("SCIM_RETRY_COUNT")
        .as_deref()
        .unwrap_or("100")
        .parse::<u16>()
        .expect("Cannot parse BACKCHANNEL_LOGOUT_RETRY_COUNT as u16");

    loop {
        // We want to randomize the sleep because this scheduler should run on all cluster members.
        // This increases the chance opf success in case of a network segmentation.
        let millis = rand::thread_rng().gen_range(5_000..10_000);
        // let millis = rand::thread_rng().gen_range(60_000..90_000);
        time::sleep(Duration::from_millis(millis)).await;

        debug!("Running scim_task_retry scheduler");
        if let Err(err) = execute(&data, retry_count).await {
            error!("Error during scim_task_retry: {}", err.message);
        }
    }
}

async fn execute(data: &web::Data<AppState>, retry_count: u16) -> Result<(), ErrorResponse> {
    let failures = FailedScimTask::find_all().await?;
    if failures.is_empty() {
        return Ok(());
    }

    let mut clients_scim: Vec<ClientScim> = Vec::with_capacity(1);
    let groups_local = Group::find_all().await?;
    let mut groups_remote = HashMap::with_capacity(groups_local.len());

    for failure in failures {
        if failure.retry_count >= retry_count as i64 {
            warn!("Retry count exceeded for scim task {:?}", failure);

            Event::scim_task_failed(&failure.client_id, &failure.action, failure.retry_count)
                .send(&data.tx_events)
                .await?;

            failure.delete().await?;
            continue;
        }

        let client_scim = if let Some(pos) = clients_scim
            .iter()
            .position(|c| c.client_id == failure.client_id)
        {
            clients_scim.get(pos).unwrap()
        } else {
            let client = ClientScim::find(failure.client_id.clone()).await?;
            clients_scim.push(client);
            clients_scim.last().unwrap()
        };

        let res = match failure.action.clone() {
            ScimAction::UserCreateUpdate(user_id) => {
                let user = User::find(user_id).await?;
                ClientScim::create_update_user(user).await
            }
            ScimAction::UserDelete(user_id) => {
                let user = User::find(user_id).await?;
                client_scim.delete_user(&user).await
            }
            ScimAction::UsersSync(last_created_ts) => {
                client_scim
                    .sync_users(Some(last_created_ts), &groups_local, &mut groups_remote)
                    .await
            }
            ScimAction::GroupCreateUpdate(group_id) => {
                let group = Group::find(group_id).await?;
                client_scim.create_update_group(group).await
            }
            ScimAction::GroupDelete(group_id) => {
                let group = Group::find(group_id).await?;
                client_scim.delete_group(group, None).await
            }
            ScimAction::GroupsSync => client_scim.sync_groups().await,
            ScimAction::Unknown => {
                error!(
                    "FailedScimTask with unknown action for scim client {}: {:?}",
                    failure.client_id, failure.action
                );
                continue;
            }
        };

        match res {
            Ok(_) => {
                info!(
                    "Success retrying failed scim task for {}",
                    client_scim.client_id
                );
                failure.delete().await?;
            }
            Err(err) => {
                error!(
                    "Error executing scim task for client {}: {}",
                    failure.client_id, err
                );
            }
        }
    }

    Ok(())
}
