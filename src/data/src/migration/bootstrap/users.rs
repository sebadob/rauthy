use crate::database::DB;
use crate::entity::groups::Group;
use crate::entity::roles::Role;
use crate::entity::user_attr::UserAttrConfigEntity;
use crate::entity::users_values::UserValues;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::{User, UserPassword};
use chrono::Utc;
use hiqlite::macros::params;
use rauthy_api_types::users::UserValuesRequest;
use rauthy_common::is_hiqlite;
use rauthy_common::password_hasher::HashPassword;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let users = bootstrap_data!(User, "users");
    let len = users.len();

    let roles = Role::find_all()
        .await?
        .into_iter()
        .map(|r| r.name)
        .collect::<Vec<_>>();
    let groups = Group::find_all()
        .await?
        .into_iter()
        .map(|g| g.name)
        .collect::<Vec<_>>();
    let attrs = UserAttrConfigEntity::find_all()
        .await?
        .into_iter()
        .map(|a| a.name)
        .collect::<Vec<_>>();

    let sql = r#"
INSERT INTO users (
    id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
    password_expires, created_at, language, user_expires
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"#;

    // We want to fake the `created_at` a tiny bit. We modify the value slightly so that every
    // user has a unique value for that field. That may be important / interesting for testing and
    // some additional validation and checks.
    let mut now = Utc::now().timestamp() - 1 - len as i64;

    for user in users {
        now += 1;

        let id = user.id.unwrap_or_else(new_store_id);
        let password = match user.password {
            UserPassword::Plain(plain) => HashPassword::hash_password(plain).await?,
            UserPassword::Argon2ID(hash) => hash,
        };

        for role in &user.roles {
            if !roles.contains(role) {
                panic!("Role `{}` for user '{}' not found.", role, user.email)
            }
        }
        let roles = user.roles.join(",");

        let groups = if let Some(g) = user.groups {
            for group in &g {
                if !groups.contains(group) {
                    panic!("Group `{}` for user '{}' not found.", group, user.email)
                }
            }
            Some(g.join(","))
        } else {
            None
        };

        let language = user.language.unwrap_or_default().to_string();

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &id,
                        &user.email,
                        user.given_name,
                        user.family_name,
                        password,
                        roles,
                        groups,
                        user.enabled,
                        user.email_verified,
                        user.user_expires,
                        now,
                        language,
                        user.user_expires
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &id,
                    &user.email,
                    &user.given_name,
                    &user.family_name,
                    &password,
                    &roles,
                    &groups,
                    &user.enabled,
                    &user.email_verified,
                    &user.user_expires,
                    &now,
                    &language,
                    &user.user_expires,
                ],
            )
            .await?;
        }

        if user.preferred_username.is_some() || user.user_values.is_some() {
            let values = if let Some(values) = user.user_values {
                UserValuesRequest {
                    birthdate: values.birthdate,
                    phone: values.phone,
                    street: values.street,
                    zip: values.zip,
                    city: values.city,
                    country: values.country,
                    tz: values.tz,
                }
            } else {
                UserValuesRequest::default()
            };

            UserValues::insert(id.clone(), values, user.preferred_username).await?;
        }

        if let Some(values) = user.attributes {
            let sql = r#"
        INSERT INTO user_attr_values (user_id, key, value)
        VALUES ($1, $2, $3)"#;

            for attr in values {
                if !attrs.contains(&attr.key) {
                    panic!(
                        "User attribute `{}` for user '{}' not found.",
                        attr.key, user.email
                    )
                }

                let v = serde_json::to_vec(&attr.value)?;
                if is_hiqlite() {
                    DB::hql().execute(sql, params!(&id, attr.key, v)).await?;
                } else {
                    DB::pg_execute(sql, &[&id, &attr.key, &v]).await?;
                }
            }
        }
    }

    info!("Migrated {len} roles.");

    Ok(())
}
