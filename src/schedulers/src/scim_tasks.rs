use rauthy_common::utils::get_rand_between;
use rauthy_data::entity::clients_scim::ClientScim;
use rauthy_data::entity::failed_scim_tasks::{FailedScimTask, ScimAction};
use rauthy_data::entity::groups::Group;
use rauthy_data::entity::scim_types::ScimGroup;
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

pub async fn scim_task_retry() {
    let mut clients_scim: Vec<ClientScim> = Vec::with_capacity(1);
    let mut groups_remote = HashMap::with_capacity(4);

    loop {
        // We want to randomize the sleep because this scheduler should run on all cluster members.
        // This increases the chance opf success in case of a network segmentation.
        let millis = get_rand_between(60_000, 90_000);
        time::sleep(Duration::from_millis(millis)).await;

        debug!("Running scim_task_retry scheduler");
        if let Err(err) = execute(&mut clients_scim, &mut groups_remote).await {
            error!("Error during scim_task_retry: {}", err.message);
        }
    }
}

async fn execute(
    clients_scim: &mut Vec<ClientScim>,
    groups_remote: &mut HashMap<String, ScimGroup>,
) -> Result<(), ErrorResponse> {
    let failures = FailedScimTask::find_all().await?;
    if failures.is_empty() {
        return Ok(());
    }

    let groups_local = Group::find_all().await?;

    for failure in failures {
        if failure.retry_count >= RauthyConfig::get().vars.scim.retry_count as i64 {
            warn!("Retry count exceeded for scim task {failure:?}");

            Event::scim_task_failed(&failure.client_id, &failure.action, failure.retry_count)
                .send()
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
                    .sync_users(Some(last_created_ts), &groups_local, groups_remote)
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
                error!(failure.client_id, ?err, "Error executing scim task",);
            }
        }
    }

    Ok(())
}
