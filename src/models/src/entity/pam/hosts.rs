use crate::database::DB;
use crate::entity::pam::users::PamUser;
use cryptr::EncValue;
use cryptr::utils::secure_random_alnum;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};

// TODO probably change ip and dns into ref tables with multi values -> Vec<_>
#[derive(Debug, Serialize, Deserialize)]
pub struct PamHost {
    pub id: String,
    pub secret: String,
    pub gid: u32,
    pub force_mfa: bool,
    pub ip: Option<String>,
    pub name: Option<String>,
    pub dns: Option<String>,
    pub notes: Option<String>,
}

impl From<hiqlite::Row<'_>> for PamHost {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        let bytes = row.get::<Vec<u8>>("secret");
        let dec = EncValue::try_from_bytes(bytes).unwrap().decrypt().unwrap();
        let secret = String::from_utf8_lossy(dec.as_ref()).to_string();

        Self {
            id: row.get("id"),
            secret,
            gid: row.get::<i64>("gid") as u32,
            force_mfa: row.get("force_mfa"),
            ip: row.get("ip"),
            name: row.get("name"),
            dns: row.get("dns"),
            notes: row.get("notes"),
        }
    }
}

impl From<tokio_postgres::Row> for PamHost {
    fn from(row: tokio_postgres::Row) -> Self {
        let bytes = row.get::<_, Vec<u8>>("secret");
        let dec = EncValue::try_from_bytes(bytes).unwrap().decrypt().unwrap();
        let secret = String::from_utf8_lossy(dec.as_ref()).to_string();

        Self {
            id: row.get("id"),
            secret,
            gid: row.get::<_, i64>("gid") as u32,
            force_mfa: row.get("force_mfa"),
            ip: row.get("ip"),
            name: row.get("name"),
            dns: row.get("dns"),
            notes: row.get("notes"),
        }
    }
}

impl PamHost {
    pub async fn insert(id: String, gid: u32) -> Result<(), ErrorResponse> {
        let secret = secure_random_alnum(64);
        let enc = EncValue::encrypt(secret.as_bytes())?.into_bytes().to_vec();

        let sql = "INSERT INTO pam_hosts (id, secret, gid, force_mfa) VALUES ($1, $2, $3, true)";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(id, enc, gid)).await?;
        } else {
            DB::pg_execute(sql, &[&id, &enc, &gid]).await?;
        }

        Ok(())
    }

    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        Ok(res)
    }
}

impl PamHost {
    #[inline]
    pub async fn is_login_allowed(&self, user: &PamUser) -> bool {
        if !self.gid == user.gid {
            let sql = "SELECT 1 FROM pam_groups_users WHERE gid = $1 AND uid = $2";
            let is_in_group = if is_hiqlite() {
                DB::hql()
                    .query_raw_one(sql, params!(self.gid, user.id))
                    .await
                    .is_ok()
            } else {
                DB::pg_query_one_row(sql, &[&self.gid, &user.id])
                    .await
                    .is_ok()
            };
            if !is_in_group {
                return false;
            }
        }

        true
    }

    #[inline]
    pub fn validate_secret(&self, secret: String) -> Result<(), ErrorResponse> {
        if self.secret == secret {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid machine secret",
            ))
        }
    }
}
