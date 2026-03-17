use crate::database::DB;
use actix_web::HttpResponse;
use cryptr::EncValue;
use cryptr::utils::secure_random_alnum;
use hiqlite_macros::params;
use rauthy_api_types::kv::{
    KVAccessResponse, KVNamespaceResponse, KVParams, KVValueRequest, KVValueResponse,
};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};

#[derive(Debug)]
pub struct KVNamespace {
    pub ns: String,
    pub public: Option<bool>,
}

impl From<hiqlite::Row<'_>> for KVNamespace {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            ns: row.get("ns"),
            public: row.get("public"),
        }
    }
}

impl From<tokio_postgres::Row> for KVNamespace {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            ns: row.get("ns"),
            public: row.get("public"),
        }
    }
}

impl KVNamespace {
    pub async fn insert(name: String, public: Option<bool>) -> Result<(), ErrorResponse> {
        let sql = "INSERT INTO kv_ns (ns, public) VALUES ($1, $2)";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(name, public)).await?;
        } else {
            DB::pg_execute(sql, &[&name, &public]).await?;
        }

        Ok(())
    }

    pub async fn find(ns: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM kv_ns WHERE ns = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(ns)).await?
        } else {
            DB::pg_query_one(sql, &[&ns]).await?
        };

        Ok(slf)
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

    pub async fn update(
        name_old: String,
        name_new: String,
        public: Option<bool>,
    ) -> Result<(), ErrorResponse> {
        let sql = "UPDATE kv_ns SET ns = $1, public = $2 WHERE ns = $3";

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(name_new, public, name_old))
                .await?;
        } else {
            DB::pg_execute(sql, &[&name_new, &public, &name_old]).await?;
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

impl KVNamespace {
    #[inline]
    pub fn validate_pub_access(&self) -> Result<(), ErrorResponse> {
        if self.public == Some(true) {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Missing Credentials",
            ))
        }
    }
}

#[derive(Debug)]
pub struct KVValue {
    pub ns: String,
    pub key: String,
    pub encrypted: Option<bool>,
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
        encrypted: Option<bool>,
        value: serde_json::Value,
    ) -> Result<(), ErrorResponse> {
        let value = value.to_string();
        let value = if encrypted == Some(true) {
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
        encrypted: Option<bool>,
        value: serde_json::Value,
    ) -> Result<(), ErrorResponse> {
        let value = value.to_string();
        let value = if encrypted == Some(true) {
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

    pub async fn upsert(ns: String, payload: KVValueRequest) -> Result<(), ErrorResponse> {
        let value = payload.value.to_string();
        let value = if payload.encrypted == Some(true) {
            EncValue::encrypt(value.as_bytes())?.into_bytes().to_vec()
        } else {
            value.as_bytes().to_vec()
        };

        let sql = r#"
INSERT INTO kv_values (ns, key, encrypted, value)
VALUES ($1, $2, $3, $4)
ON CONFLICT (ns, key) DO UPDATE SET encrypted = $3, value = $4
"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(ns, payload.key, payload.encrypted, value))
                .await?;
        } else {
            DB::pg_execute(sql, &[&ns, &payload.key, &payload.encrypted, &value]).await?;
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
            let sql = r#"
SELECT * FROM kv_values
WHERE ns = $1 AND key LIKE $2
ORDER BY key
LIMIT $3"#;
            let like = format!("%{}%", search);
            if is_hiqlite() {
                DB::hql().query_map(sql, params!(ns, like, limit)).await?
            } else {
                DB::pg_query(sql, &[&ns, &like, &(limit as i64)], 0).await?
            }
        } else {
            let sql = "SELECT * FROM kv_values WHERE ns = $1 ORDER BY key LIMIT $2";
            if is_hiqlite() {
                DB::hql().query_map(sql, params!(ns, limit)).await?
            } else {
                DB::pg_query(sql, &[&ns, &(limit as i64)], 0).await?
            }
        };

        Ok(res)
    }

    pub async fn find_all_keys(ns: String, params: KVParams) -> Result<Vec<String>, ErrorResponse> {
        let limit = params.limit.unwrap_or(u32::MAX);

        let res = if let Some(search) = params.search {
            let sql = r#"
SELECT key FROM kv_values
WHERE ns = $1 AND key LIKE $2
ORDER BY key
LIMIT $3
"#;
            let like = format!("%{}%", search);
            if is_hiqlite() {
                DB::hql()
                    .query_raw(sql, params!(ns, like, limit))
                    .await?
                    .into_iter()
                    .map(|mut row| row.get::<String>("key"))
                    .collect()
            } else {
                DB::pg_query_rows(sql, &[&ns, &like, &(limit as i64)], 0)
                    .await?
                    .into_iter()
                    .map(|row| row.get::<_, String>("key"))
                    .collect()
            }
        } else {
            let sql = r#"
SELECT key FROM kv_values
WHERE ns = $1
ORDER BY key
LIMIT $2
"#;

            if is_hiqlite() {
                DB::hql()
                    .query_raw(sql, params!(ns, limit))
                    .await?
                    .into_iter()
                    .map(|mut row| row.get::<String>("key"))
                    .collect()
            } else {
                DB::pg_query_rows(sql, &[&ns, &(limit as i64)], 0)
                    .await?
                    .into_iter()
                    .map(|row| row.get::<_, String>("key"))
                    .collect()
            }
        };

        Ok(res)
    }
}

impl KVValue {
    #[inline]
    pub fn value(&self) -> Result<serde_json::Value, ErrorResponse> {
        let v = if self.encrypted == Some(true) {
            let bytes = EncValue::try_from(self.value.clone()).unwrap().decrypt()?;
            serde_json::from_slice(bytes.as_ref())?
        } else {
            serde_json::from_slice(&self.value)?
        };
        Ok(v)
    }

    #[inline]
    pub fn value_response(self) -> Result<HttpResponse, ErrorResponse> {
        let v = self.value()?;
        match v {
            serde_json::Value::Object(v) => Ok(HttpResponse::Ok().json(v)),
            serde_json::Value::Array(v) => Ok(HttpResponse::Ok().json(v)),
            serde_json::Value::String(s) => {
                // The serialized string will have parenthesis. This makes sense as a JSON value,
                // but is not very usable in this context, so we want to get rid of them.
                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(s))
            }
            other => Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(other.to_string())),
        }
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
    pub secret: Vec<u8>,
    pub enabled: bool,
    pub name: Option<String>,
}

impl From<hiqlite::Row<'_>> for KVAccess {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            id: row.get("id"),
            ns: row.get("ns"),
            secret: row.get("secret"),
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
            secret: row.get("secret"),
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
    ) -> Result<KVAccessResponse, ErrorResponse> {
        let id = secure_random_alnum(16);
        // 48 characters is enough for the secret. The `id` is a secure random as well,
        // and you need to know both.
        let secret_pain = secure_random_alnum(48);
        let secret = EncValue::encrypt(secret_pain.as_bytes())?
            .into_bytes()
            .to_vec();

        let sql =
            "INSERT INTO kv_access (id, ns, secret, enabled, name) VALUES ($1, $2, $3, $4, $5)";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&id, &ns, secret, enabled, &name))
                .await?;
        } else {
            DB::pg_execute(sql, &[&id, &ns, &secret, &enabled, &name]).await?;
        }

        Ok(KVAccessResponse {
            id,
            ns,
            secret: secret_pain,
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

    pub async fn generate_new_secret(self) -> Result<KVAccessResponse, ErrorResponse> {
        let secret_pain = secure_random_alnum(48);
        let secret = EncValue::encrypt(secret_pain.as_bytes())?
            .into_bytes()
            .to_vec();

        let sql = "UPDATE kv_access SET secret = $1 WHERE id = $2";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(secret, &self.id)).await?;
        } else {
            DB::pg_execute(sql, &[&secret, &self.id]).await?;
        }

        Ok(KVAccessResponse {
            id: self.id,
            ns: self.ns,
            secret: secret_pain,
            enabled: self.enabled,
            name: self.name,
        })
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

    /// Finds the KV Access Key from the given `Bearer` and validates it.
    pub async fn find_validated(bearer: &str) -> Result<Self, ErrorResponse> {
        let Some((id, secret)) = bearer.split_once('$') else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid Access Key",
            ));
        };

        let sql = "SELECT * FROM kv_access WHERE id = $1";
        let slf: Self = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        if !slf.enabled {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Disabled Access Key",
            ));
        }

        slf.validate_secret(secret)?;

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

impl KVAccess {
    pub fn into_response(self) -> Result<KVAccessResponse, ErrorResponse> {
        let dec = EncValue::try_from(self.secret)?.decrypt()?;

        Ok(KVAccessResponse {
            id: self.id,
            ns: self.ns,
            secret: String::from_utf8_lossy(dec.as_ref()).into(),
            enabled: self.enabled,
            name: self.name,
        })
    }

    #[inline]
    pub fn validate_secret(&self, secret: &str) -> Result<(), ErrorResponse> {
        let plain = EncValue::try_from(self.secret.clone()).unwrap().decrypt()?;
        if constant_time_eq::constant_time_eq(secret.as_bytes(), plain.as_ref()) {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid Credentials",
            ))
        }
    }
}

impl From<KVNamespace> for KVNamespaceResponse {
    fn from(value: KVNamespace) -> Self {
        Self {
            name: value.ns,
            public: value.public,
        }
    }
}
