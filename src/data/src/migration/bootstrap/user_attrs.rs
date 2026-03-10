use crate::database::DB;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::UserAttribute;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let attrs = bootstrap_data!(UserAttribute, "user_attributes");

    let len = attrs.len();

    // Note: The queries do currently NOT include the `typ`, because it's still unused.

    for attr in attrs {
        let default_value = if let Some(bytes) = &attr.default_value {
            Some(serde_json::to_vec(bytes)?)
        } else {
            None
        };

        if is_hiqlite() {
            DB::hql()
                .execute(
                    r#"
INSERT INTO user_attr_config (name, desc, default_value, user_editable)
VALUES ($1, $2, $3, $4)"#,
                    params!(attr.name, attr.desc, default_value, attr.user_editable),
                )
                .await?;
        } else {
            DB::pg_execute(
                r#"
INSERT INTO user_attr_config (name, "desc", default_value, user_editable)
VALUES ($1, $2, $3, $4)"#,
                &[&attr.name, &attr.desc, &default_value, &attr.user_editable],
            )
            .await?;
        };
    }

    info!("Migrated {len} user_attributes.");

    Ok(())
}
