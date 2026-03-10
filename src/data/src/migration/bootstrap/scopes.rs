use crate::database::DB;
use crate::entity::user_attr::UserAttrConfigEntity;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::Scope;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let scopes = bootstrap_data!(Scope, "scopes");
    let len = scopes.len();

    let attr_names = UserAttrConfigEntity::find_all()
        .await?
        .into_iter()
        .map(|attr| attr.name)
        .collect::<Vec<_>>();

    let sql = r#"
INSERT INTO scopes (id, name, attr_include_access, attr_include_id)
VALUES ($1, $2, $3, $4)"#;

    for scope in scopes {
        let attr_include_access = build_attrs(
            &attr_names,
            &scope,
            &scope.attr_include_access,
            "attr_include_access",
        );
        let attr_include_id = build_attrs(
            &attr_names,
            &scope,
            &scope.attr_include_id,
            "attr_include_id",
        );

        let id = scope.id.unwrap_or_else(new_store_id);

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(id, scope.name, attr_include_access, attr_include_id),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&id, &scope.name, &attr_include_access, &attr_include_id],
            )
            .await?;
        }
    }

    info!("Migrated {len} scopes.");

    Ok(())
}

#[cold]
fn build_attrs(
    attr_names: &[String],
    scope: &Scope,
    attrs: &Option<Vec<String>>,
    debug_name: &str,
) -> Option<String> {
    if let Some(names) = attrs {
        for name in names {
            if name.is_empty() {
                panic!(
                    "Invalid bootstrap data for {:?}. Empty User Attribute name in `{}`",
                    scope, debug_name
                );
            }
            if !attr_names.contains(name) {
                panic!(
                    "Invalid bootstrap data for {:?}. User Attribute name in `{}` does not exist",
                    scope, debug_name
                );
            }
        }
        Some(names.join(","))
    } else {
        None
    }
}
