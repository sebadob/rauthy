use crate::database::DB;
use cryptr::EncValue;
use cryptr::utils::secure_random_alnum;
use hiqlite_macros::params;
use rauthy_api_types::kv::{
    KVAccessResponse, KVNamespaceResponse, KVParams, KVValueRequest, KVValueResponse,
};
use rauthy_common::constants::API_KEY_LENGTH;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;

#[derive(Debug)]
pub struct KVNamespace {
    pub ns: String,
}

impl From<hiqlite::Row<'_>> for KVNamespace {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self { ns: row.get("ns") }
    }
}

impl From<tokio_postgres::Row> for KVNamespace {
    fn from(row: tokio_postgres::Row) -> Self {
        Self { ns: row.get("ns") }
    }
}

impl KVNamespace {
    pub async fn insert(name: String) -> Result<(), ErrorResponse> {
        let sql = "INSERT INTO kv_ns (ns) VALUES ($1)";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(name)).await?;
        } else {
            DB::pg_execute(sql, &[&name]).await?;
        }

        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM kv_ns";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 1).await?
        };

        Ok(res)
    }

    pub async fn update(name_old: String, name_new: String) -> Result<(), ErrorResponse> {
        let sql = "UPDATE kv_ns SET ns = $1 WHERE ns = $2";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(name_new, name_old)).await?;
        } else {
            DB::pg_execute(sql, &[&name_new, &name_old]).await?;
        }

        Ok(())
    }

    pub async fn delete(name: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM kv_ns WHERE ns = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(name)).await?;
        } else {
            DB::pg_execute(sql, &[&name]).await?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct KVValue {
    pub ns: String,
    pub key: String,
    pub encrypted: bool,
    pub value: Vec<u8>,
}

impl From<hiqlite::Row<'_>> for KVValue {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            ns: row.get("ns"),
            key: row.get("key"),
            encrypted: row.get("encrypted"),
            value: row.get("value"),
        }
    }
}

impl From<tokio_postgres::Row> for KVValue {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            ns: row.get("ns"),
            key: row.get("key"),
            encrypted: row.get("encrypted"),
            value: row.get("value"),
        }
    }
}

impl KVValue {
    pub async fn insert(
        ns: String,
        key: String,
        encrypted: bool,
        value: serde_json::Value,
    ) -> Result<(), ErrorResponse> {
        let value = value.to_string();
        let value = if encrypted {
            EncValue::encrypt(value.as_bytes())?.into_bytes().to_vec()
        } else {
            value.as_bytes().to_vec()
        };

        let sql = "INSERT INTO kv_values (ns, key, encrypted, value) VALUES ($1, $2, $3, $4)";

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(ns, key, encrypted, value))
                .await?;
        } else {
            DB::pg_execute(sql, &[&ns, &key, &encrypted, &value]).await?;
        }

        Ok(())
    }

    pub async fn update(
        ns: String,
        key: String,
        encrypted: bool,
        value: serde_json::Value,
    ) -> Result<(), ErrorResponse> {
        let value = value.to_string();
        let value = if encrypted {
            EncValue::encrypt(value.as_bytes())?.into_bytes().to_vec()
        } else {
            value.as_bytes().to_vec()
        };

        let sql = "UPDATE kv_values SET encrypted = $1, value = $2 WHERE ns = $3 AND key = $4";

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(encrypted, value, ns, key))
                .await?;
        } else {
            DB::pg_execute(sql, &[&encrypted, &value, &ns, &key]).await?;
        }

        Ok(())
    }

    pub async fn upsert_by_access_id(
        access_id: String,
        payload: KVValueRequest,
    ) -> Result<(), ErrorResponse> {
        let value = payload.value.to_string();
        let value = if payload.encrypted {
            EncValue::encrypt(value.as_bytes())?.into_bytes().to_vec()
        } else {
            value.as_bytes().to_vec()
        };

        let sql = r#"
INSERT INTO kv_values (ns, key, encrypted, value)
VALUES (
    (SELECT ns FROM kv_access WHERE enabled = true AND id = $1),
    $2, $3, $4
)
ON CONFLICT (ns, key) DO UPDATE SET encrypted = $3, value = $4
"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(access_id, payload.key, payload.encrypted, value),
                )
                .await?;
        } else {
            DB::pg_execute(sql, &[&access_id, &payload.key, &payload.encrypted, &value]).await?;
        }

        Ok(())
    }

    pub async fn delete(ns: String, key: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM kv_values WHERE ns = $1 AND key = $2";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(ns, key)).await?;
        } else {
            DB::pg_execute(sql, &[&ns, &key]).await?;
        }
        Ok(())
    }

    pub async fn delete_by_access_id(access_id: String, key: String) -> Result<(), ErrorResponse> {
        let sql = r#"
DELETE FROM kv_values
WHERE ns = (SELECT ns FROM kv_access WHERE enabled = true AND id = $1) AND key = $2
"#;

        if is_hiqlite() {
            DB::hql().execute(sql, params!(access_id, key)).await?
        } else {
            DB::pg_execute(sql, &[&access_id, &key]).await?
        };

        Ok(())
    }

    pub async fn find(ns: String, key: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM kv_values WHERE ns = $1 AND key = $2";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(ns, key)).await?
        } else {
            DB::pg_query_one(sql, &[&ns, &key]).await?
        };

        Ok(slf)
    }

    pub async fn find_all(ns: String, params: KVParams) -> Result<Vec<Self>, ErrorResponse> {
        let limit = params.limit.unwrap_or(u32::MAX);

        let res = if let Some(search) = params.search {
            let sql = "SELECT * FROM kv_values WHERE ns = $1 AND key LIKE $2 LIMIT $3";
            let like = format!("%{}%", search);
            if is_hiqlite() {
                DB::hql().query_map(sql, params!(ns, like, limit)).await?
            } else {
                DB::pg_query(sql, &[&ns, &like, &limit], 0).await?
            }
        } else {
            let sql = "SELECT * FROM kv_values WHERE ns = $1 LIMIT $2";
            if is_hiqlite() {
                DB::hql().query_map(sql, params!(ns, limit)).await?
            } else {
                DB::pg_query(sql, &[&ns, &limit], 0).await?
            }
        };

        Ok(res)
    }

    pub async fn find_by_access_id(access_id: String, key: String) -> Result<Self, ErrorResponse> {
        let sql = r#"
SELECT * FROM kv_values
WHERE ns = (SELECT ns FROM kv_access WHERE enabled = true AND id = $1) AND key = $2
"#;

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_one(sql, params!(access_id, key))
                .await?
        } else {
            DB::pg_query_one(sql, &[&access_id, &key]).await?
        };

        Ok(slf)
    }

    pub async fn find_all_by_access_id(
        access_id: String,
        params: KVParams,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let limit = params.limit.unwrap_or(u32::MAX);

        let res = if let Some(search) = params.search {
            let sql = r#"
SELECT * FROM kv_values
WHERE ns = (SELECT ns FROM kv_access WHERE enabled = true AND id = $1) AND key LIKE $2
LIMIT $3
"#;
            let like = format!("%{}%", search);

            if is_hiqlite() {
                DB::hql()
                    .query_map(sql, params!(access_id, like, limit))
                    .await?
            } else {
                DB::pg_query(sql, &[&access_id, &like, &limit], 0).await?
            }
        } else {
            let sql = r#"
SELECT * FROM kv_values
WHERE ns = (SELECT ns FROM kv_access WHERE enabled = true AND id = $1)
LIMIT $2
"#;

            if is_hiqlite() {
                DB::hql().query_map(sql, params!(access_id, limit)).await?
            } else {
                DB::pg_query(sql, &[&access_id, &limit], 0).await?
            }
        };

        Ok(res)
    }

    pub async fn find_all_keys_by_access_id(
        access_id: String,
        limit: Option<u32>,
    ) -> Result<Vec<String>, ErrorResponse> {
        let limit = limit.unwrap_or(u32::MAX);
        let sql = r#"
SELECT key FROM kv_values
WHERE ns = (SELECT ns FROM kv_access WHERE enabled = true AND id = $1)
LIMIT $2
"#;

        let res = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!(access_id, limit))
                .await?
                .into_iter()
                .map(|mut row| row.get::<String>("key"))
                .collect()
        } else {
            DB::pg_query_rows(sql, &[&access_id, &limit], 0)
                .await?
                .into_iter()
                .map(|row| row.get::<_, String>("key"))
                .collect()
        };

        Ok(res)
    }
}

impl KVValue {
    #[inline]
    pub fn value(&self) -> Result<serde_json::Value, ErrorResponse> {
        let v = if self.encrypted {
            let bytes = EncValue::try_from(self.value.clone()).unwrap().decrypt()?;
            serde_json::from_slice(bytes.as_ref())?
        } else {
            serde_json::from_slice(&self.value)?
        };
        Ok(v)
    }

    #[inline]
    pub fn try_into_response(self) -> Result<KVValueResponse, ErrorResponse> {
        let value = self.value()?;
        Ok(KVValueResponse {
            key: self.key,
            encrypted: self.encrypted,
            value,
        })
    }
}

#[derive(Debug)]
pub struct KVAccess {
    pub id: String,
    pub ns: String,
    pub enabled: bool,
    pub name: Option<String>,
}

impl From<hiqlite::Row<'_>> for KVAccess {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            id: row.get("id"),
            ns: row.get("ns"),
            enabled: row.get("enabled"),
            name: row.get("name"),
        }
    }
}

impl From<tokio_postgres::Row> for KVAccess {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            ns: row.get("ns"),
            enabled: row.get("enabled"),
            name: row.get("name"),
        }
    }
}

impl KVAccess {
    pub async fn insert(
        ns: String,
        enabled: bool,
        name: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let id = secure_random_alnum(API_KEY_LENGTH);

        let sql = "INSERT INTO kv_access (id, ns, enabled, name) VALUES ($1, $2, $3, $4)";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&id, &ns, enabled, &name))
                .await?;
        } else {
            DB::pg_execute(sql, &[&id, &ns, &enabled, &name]).await?;
        }

        Ok(Self {
            id,
            ns,
            enabled,
            name,
        })
    }

    pub async fn update(
        id: String,
        enabled: bool,
        name: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let sql = "UPDATE kv_access SET enabled = $1, name = $2 WHERE id = $3";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(enabled, name, id)).await?;
        } else {
            DB::pg_execute(sql, &[&enabled, &name, &id]).await?;
        }
        Ok(())
    }

    pub async fn delete(id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM kv_access WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }
        Ok(())
    }

    pub async fn generate_new_secret(id: String) -> Result<(), ErrorResponse> {
        let id_new = secure_random_alnum(API_KEY_LENGTH);

        let sql = "UPDATE kv_access SET id = $1 WHERE id = $2";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id_new, id)).await?;
        } else {
            DB::pg_execute(sql, &[&id_new, &id]).await?;
        }

        Ok(())
    }

    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM kv_access WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_all(ns: String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM kv_access WHERE ns = $1";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(ns)).await?
        } else {
            DB::pg_query(sql, &[&ns], 0).await?
        };
        Ok(res)
    }
}

impl From<KVNamespace> for KVNamespaceResponse {
    fn from(value: KVNamespace) -> Self {
        Self { name: value.ns }
    }
}

impl From<KVAccess> for KVAccessResponse {
    fn from(value: KVAccess) -> Self {
        Self {
            id: value.id,
            ns: value.ns,
            enabled: value.enabled,
            name: value.name,
        }
    }
}
