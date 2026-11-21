use crate::database::DB;
use crate::entity::scopes::Scope as ScopeEntity;
use crate::entity::user_attr::UserAttrConfigEntity;
use crate::migration::bootstrap::types::Scope;
use crate::migration::bootstrap::{bootstrap_data, find_existing_entry};
use hiqlite::macros::params;
use rauthy_api_types::scopes::ScopeRequest;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::{info, warn};

pub async fn load() -> Result<Vec<Scope>, ErrorResponse> {
    Ok(bootstrap_data!(Scope, "scopes"))
}

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let scopes = load().await?;
    let len = scopes.len();
    let attr_names = attr_names().await?;

    for scope in scopes {
        insert(&attr_names, scope).await?;
    }

    info!("Migrated {len} scopes.");

    Ok(())
}

/// Creates missing and updates changed scopes. Never deletes anything.
pub async fn reconcile(scopes: Vec<Scope>) -> Result<(), ErrorResponse> {
    let existing = ScopeEntity::find_all().await?;
    let attr_names = attr_names().await?;
    let mut created = 0;
    let mut updated = 0;

    for scope in scopes {
        let Some(e) = find_existing_entry(&existing, scope.id.as_deref(), &scope.name, |e| {
            (&e.id, &e.name)
        }) else {
            insert(&attr_names, scope).await?;
            created += 1;
            continue;
        };

        let values = build_values(&attr_names, &scope);
        if is_unchanged(e, &scope.name, &values) {
            continue;
        }
        // `update()` rejects changes to the default OIDC scopes
        if !ScopeEntity::is_custom(&e.name) {
            warn!(
                "Bootstrap scope '{}' differs from the database, but default OIDC scopes are not \
                reconciled",
                e.name
            );
            continue;
        }
        let claims_at_root = values.2;

        // `update()` also updates the clients using this scope on a rename
        ScopeEntity::update(
            &e.id,
            ScopeRequest {
                scope: scope.name,
                attr_include_access: scope.attr_include_access,
                attr_include_id: scope.attr_include_id,
                claims_at_root,
            },
        )
        .await?;
        updated += 1;
    }

    if created > 0 {
        // `insert()` bypasses the entity and therefore its cache
        ScopeEntity::clear_cache().await?;
    }
    info!("Reconciled scopes: {created} created, {updated} updated.");

    Ok(())
}

async fn attr_names() -> Result<Vec<String>, ErrorResponse> {
    Ok(UserAttrConfigEntity::find_all()
        .await?
        .into_iter()
        .map(|attr| attr.name)
        .collect())
}

fn is_unchanged(
    e: &ScopeEntity,
    name: &str,
    values: &(Option<String>, Option<String>, bool),
) -> bool {
    e.name == name
        && e.attr_include_access == values.0
        && e.attr_include_id == values.1
        && e.claims_at_root == values.2
}

/// Returns `(attr_include_access, attr_include_id, claims_at_root)` as they are stored.
fn build_values(attr_names: &[String], scope: &Scope) -> (Option<String>, Option<String>, bool) {
    let attr_include_access = build_attrs(
        attr_names,
        scope,
        &scope.attr_include_access,
        "attr_include_access",
    );
    let attr_include_id = build_attrs(attr_names, scope, &scope.attr_include_id, "attr_include_id");

    // `claims_at_root` is only meaningful for custom scopes; normalize it so a
    // default OIDC scope can never be stored as requesting root emission.
    let claims_at_root =
        scope.claims_at_root.unwrap_or(false) && ScopeEntity::is_custom(&scope.name);

    (attr_include_access, attr_include_id, claims_at_root)
}

async fn insert(attr_names: &[String], scope: Scope) -> Result<(), ErrorResponse> {
    let sql = r#"
INSERT INTO scopes (id, name, attr_include_access, attr_include_id, claims_at_root)
VALUES ($1, $2, $3, $4, $5)"#;

    let (attr_include_access, attr_include_id, claims_at_root) = build_values(attr_names, &scope);
    let id = scope.id.unwrap_or_else(new_store_id);

    if is_hiqlite() {
        DB::hql()
            .execute(
                sql,
                params!(
                    id,
                    scope.name,
                    attr_include_access,
                    attr_include_id,
                    claims_at_root
                ),
            )
            .await?;
    } else {
        DB::pg_execute(
            sql,
            &[
                &id,
                &scope.name,
                &attr_include_access,
                &attr_include_id,
                &claims_at_root,
            ],
        )
        .await?;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unchanged() {
        let e = ScopeEntity {
            id: "id".to_string(),
            name: "custom".to_string(),
            attr_include_access: Some("a,b".to_string()),
            attr_include_id: None,
            claims_at_root: false,
        };
        let same = (Some("a,b".to_string()), None, false);

        assert!(is_unchanged(&e, "custom", &same));
        assert!(!is_unchanged(&e, "renamed", &same));
        assert!(!is_unchanged(
            &e,
            "custom",
            &(Some("a".to_string()), None, false)
        ));
        assert!(!is_unchanged(
            &e,
            "custom",
            &(Some("a,b".to_string()), Some(String::new()), false)
        ));
        assert!(!is_unchanged(
            &e,
            "custom",
            &(Some("a,b".to_string()), None, true)
        ));
    }
}
