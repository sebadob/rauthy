use crate::database::DB;
use cryptr::EncValue;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;

#[derive(Debug)]
pub struct ClientScim {
    pub client_id: String,
    pub bearer_token: String,
    pub base_endpoint: String,
    pub group_sync_prefix: Option<String>,
}

impl From<hiqlite::Row<'_>> for ClientScim {
    fn from(mut row: hiqlite::Row) -> ClientScim {
        let bearer_token = Self::decrypt_bearer_token(row.get("bearer_token"))
            .expect("Column clients_scim.bearer_token corrupted");
        Self {
            client_id: row.get("client_id"),
            bearer_token,
            base_endpoint: row.get("base_endpoint"),
            group_sync_prefix: row.get("group_sync_prefix"),
        }
    }
}

impl From<tokio_postgres::Row> for ClientScim {
    fn from(row: tokio_postgres::Row) -> ClientScim {
        let bearer_token = Self::decrypt_bearer_token(row.get("bearer_token"))
            .expect("Column clients_scim.bearer_token corrupted");
        Self {
            client_id: row.get("client_id"),
            bearer_token,
            base_endpoint: row.get("base_endpoint"),
            group_sync_prefix: row.get("group_sync_prefix"),
        }
    }
}

impl ClientScim {
    pub async fn upsert(
        client_id: String,
        bearer_token: &[u8],
        base_endpoint: String,
        group_sync_prefix: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let bearer_encrypted = EncValue::encrypt(bearer_token)?.into_bytes();
        let sql = r#"
INSERT INTO clients_scim (client_id, bearer_token, base_endpoint, group_sync_prefix)
VALUES ($1, $2, $3, $4)
ON CONFLICT (client_id) DO UPDATE
    SET bearer_token = $2, base_endpoint = $3, group_sync_prefix = $4"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        client_id,
                        bearer_encrypted.to_vec(),
                        base_endpoint,
                        group_sync_prefix
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &client_id,
                    &bearer_encrypted.as_ref(),
                    &base_endpoint,
                    &group_sync_prefix,
                ],
            )
            .await?;
        }

        Ok(())
    }

    pub async fn find(client_id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM clients_scim WHERE client_id = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(client_id)).await?
        } else {
            DB::pg_query_one(sql, &[&client_id]).await?
        };
        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM clients_scim";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn delete(client_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM clients_scim WHERE client_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(client_id)).await?;
        } else {
            DB::pg_execute(sql, &[&client_id]).await?;
        }
        Ok(())
    }
}

impl ClientScim {
    fn decrypt_bearer_token(encrypted_token: Vec<u8>) -> Result<String, ErrorResponse> {
        let bytes = EncValue::try_from(encrypted_token)?.decrypt()?;
        let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
        Ok(cleartext)
    }
}
