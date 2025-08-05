use crate::database::DB;
use crate::entity::pam::users::PamUser;
use cryptr::EncValue;
use cryptr::utils::secure_random_alnum;
use hiqlite_macros::params;
use rauthy_api_types::pam::{
    PamGroupMembersResponse, PamGroupType, PamHostAccessResponse, PamHostDetailsResponse,
    PamHostSimpleResponse, PamHostUpdateRequest,
};
use rauthy_common::constants::{PAM_WHEEL_ID, PAM_WHEEL_NAME};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct PamHost {
    pub id: String,
    pub hostname: String,
    pub gid: u32,
    pub secret: String,
    pub force_mfa: bool,
    pub local_password_only: bool,
    pub notes: Option<String>,
    pub ips: Vec<IpAddr>,
    pub aliases: Vec<String>,
}

impl From<hiqlite::Row<'_>> for PamHost {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        let bytes = row.get::<Vec<u8>>("secret");
        let dec = EncValue::try_from_bytes(bytes).unwrap().decrypt().unwrap();
        let secret = String::from_utf8_lossy(dec.as_ref()).to_string();

        Self {
            id: row.get("id"),
            hostname: row.get("hostname"),
            gid: row.get::<i64>("gid") as u32,
            secret,
            force_mfa: row.get("force_mfa"),
            local_password_only: row.get("local_password_only"),
            notes: row.get("notes"),
            ips: Vec::default(),
            aliases: Vec::default(),
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
            hostname: row.get("hostname"),
            gid: row.get::<_, i64>("gid") as u32,
            secret,
            force_mfa: row.get("force_mfa"),
            local_password_only: row.get("local_password_only"),
            notes: row.get("notes"),
            ips: Vec::default(),
            aliases: Vec::default(),
        }
    }
}

impl PamHost {
    pub async fn insert(
        hostname: String,
        gid: u32,
        force_mfa: bool,
        local_password_only: bool,
    ) -> Result<Self, ErrorResponse> {
        let id = secure_random_alnum(24);
        let secret = secure_random_alnum(64);
        let enc = EncValue::encrypt(secret.as_bytes())?.into_bytes().to_vec();

        let sql = r#"
INSERT INTO pam_hosts (id, hostname, gid, secret, force_mfa, local_password_only)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *
"#;

        let slf = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(
                    sql,
                    params!(id, hostname, gid, enc, force_mfa, local_password_only),
                )
                .await?
        } else {
            DB::pg_query_one(
                sql,
                &[
                    &id,
                    &hostname,
                    &(gid as i64),
                    &enc,
                    &force_mfa,
                    &local_password_only,
                ],
            )
            .await?
        };

        Ok(slf)
    }

    /// Will always return empty `ips` and `aliases`
    pub async fn find_simple(host_id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(host_id)).await?
        } else {
            DB::pg_query_one(sql, &[&host_id]).await?
        };

        Ok(slf)
    }

    #[inline(always)]
    async fn find_aliases(host_id: String) -> Result<Vec<String>, ErrorResponse> {
        let sql = "SELECT alias FROM pam_hosts_aliases WHERE host_id = $1";

        let aliases = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!(host_id))
                .await?
                .into_iter()
                .map(|mut r| r.get::<String>("alias"))
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[&host_id], 0)
                .await?
                .into_iter()
                .map(|r| r.get::<_, String>("alias"))
                .collect::<Vec<_>>()
        };

        Ok(aliases)
    }

    // Find's all IPs and returns `<host_id, Vec<alias>>`
    #[inline(always)]
    async fn find_aliases_all() -> Result<BTreeMap<String, Vec<String>>, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts_aliases";

        let aliases = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!())
                .await?
                .into_iter()
                .map(|mut r| {
                    let host_id = r.get::<String>("host_id");
                    let alias = r.get::<String>("alias");
                    (host_id, alias)
                })
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[], 16)
                .await?
                .into_iter()
                .map(|r| {
                    let host_id = r.get::<_, String>("host_id");
                    let alias = r.get::<_, String>("alias");
                    (host_id, alias)
                })
                .collect::<Vec<_>>()
        };

        let mut res: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (host_id, alias) in aliases {
            if let Some(values) = res.get_mut(&host_id) {
                values.push(alias);
            } else {
                res.insert(host_id, vec![alias]);
            }
        }

        Ok(res)
    }

    #[inline(always)]
    async fn find_ips(host_id: String) -> Result<Vec<IpAddr>, ErrorResponse> {
        let sql = "SELECT ip FROM pam_hosts_ips WHERE host_id = $1";

        let ips = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!(host_id))
                .await?
                .into_iter()
                .filter_map(|mut r| r.get::<String>("ip").parse::<IpAddr>().ok())
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[&host_id], 2)
                .await?
                .into_iter()
                .filter_map(|r| r.get::<_, String>("ip").parse::<IpAddr>().ok())
                .collect::<Vec<_>>()
        };

        Ok(ips)
    }

    // Find's all IPs and returns `<host_id, Vec<ip>>`
    #[inline(always)]
    async fn find_ips_all() -> Result<BTreeMap<String, Vec<IpAddr>>, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts_ips";

        let ips = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!())
                .await?
                .into_iter()
                .filter_map(|mut r| {
                    if let Ok(ip) = r.get::<String>("ip").parse::<IpAddr>() {
                        let host_id = r.get::<String>("host_id");
                        Some((host_id, ip))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[], 16)
                .await?
                .into_iter()
                .filter_map(|r| {
                    if let Ok(ip) = r.get::<_, String>("ip").parse::<IpAddr>() {
                        let host_id = r.get::<_, String>("host_id");
                        Some((host_id, ip))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        let mut res: BTreeMap<String, Vec<IpAddr>> = BTreeMap::new();
        for (host_id, ip) in ips {
            if let Some(values) = res.get_mut(&host_id) {
                values.push(ip);
            } else {
                res.insert(host_id, vec![ip]);
            }
        }

        Ok(res)
    }

    pub async fn find_by_id_full(id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts WHERE id = $1";

        let mut slf: Self = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        // We don't have them in a single query with some aggregate function because it would be
        // a bit less efficient to build up a bigger string first and then destructure and parse
        // it again. By default, Rauthy uses Hiqlite, which means there's no network latency added
        // for DB queries.
        slf.aliases = Self::find_aliases(slf.id.clone()).await?;
        slf.ips = Self::find_ips(slf.id.clone()).await?;

        Ok(slf)
    }

    pub async fn find_by_ip_full(ip: IpAddr) -> Result<Self, ErrorResponse> {
        let sql = r#"
SELECT * FROM pam_hosts WHERE id = (
    SELECT host_id FROM pam_hosts_ips
    WHERE ip = $1
    LIMIT 1
)
"#;
        let ip = ip.to_string();

        let mut slf: Self = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(ip)).await?
        } else {
            DB::pg_query_one(sql, &[&ip]).await?
        };

        // We don't have them in a single query with some aggregate function because it would be
        // a bit less efficient to build up a bigger string first and then destructure and parse
        // it again. By default, Rauthy uses Hiqlite, which means there's no network latency added
        // for DB queries.
        slf.aliases = Self::find_aliases(slf.id.clone()).await?;
        slf.ips = Self::find_ips(slf.id.clone()).await?;

        Ok(slf)
    }

    pub async fn find_by_alias_full(hostname: String) -> Result<Self, ErrorResponse> {
        let sql = r#"
SELECT * FROM pam_hosts WHERE hostname = $1 OR id = (
    SELECT host_id FROM pam_hosts_aliases
    WHERE alias = $1
    LIMIT 1
)
LIMIT 1
"#;

        let mut slf: Self = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(hostname)).await?
        } else {
            DB::pg_query_one(sql, &[&hostname]).await?
        };

        // We don't have them in a single query with some aggregate function because it would be
        // a bit less efficient to build up a bigger string first and then destructure and parse
        // it again. By default, Rauthy uses Hiqlite, which means there's no network latency added
        // for DB queries.
        slf.aliases = Self::find_aliases(slf.id.clone()).await?;
        slf.ips = Self::find_ips(slf.id.clone()).await?;

        Ok(slf)
    }

    pub async fn count_with_group(gid: u32) -> Result<i64, ErrorResponse> {
        let sql = "SELECT COUNT(*) AS count FROM pam_hosts WHERE gid = $1";

        let count: i64 = if is_hiqlite() {
            let mut row = DB::hql().query_raw_one(sql, params!(gid)).await?;
            row.get("count")
        } else {
            let row = DB::pg_query_one_row(sql, &[&(gid as i64)]).await?;
            row.get("count")
        };

        Ok(count)
    }

    pub async fn find_in_group_full(gid: u32) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts WHERE gid = $1";

        let mut hosts: Vec<Self> = if is_hiqlite() {
            DB::hql().query_map(sql, params!(gid)).await?
        } else {
            DB::pg_query(sql, &[&(gid as i64)], 8).await?
        };

        let aliases = Self::find_aliases_all().await?;
        let ips = Self::find_ips_all().await?;

        for host in hosts.iter_mut() {
            if let Some(a) = aliases.get(&host.id) {
                host.aliases = a.clone();
            }
            if let Some(i) = ips.get(&host.id) {
                host.ips = i.clone();
            }
        }

        Ok(hosts)
    }

    /// Finds basic host data.
    pub async fn find_all_simple() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        Ok(res)
    }

    /// Finds hosts with all details, JOINs with IPs and Aliases.
    pub async fn find_all_full() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_hosts";

        let mut hosts: Vec<Self> = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        let aliases = Self::find_aliases_all().await?;
        let ips = Self::find_ips_all().await?;

        for host in hosts.iter_mut() {
            if let Some(a) = aliases.get(&host.id) {
                host.aliases = a.clone();
            }
            if let Some(i) = ips.get(&host.id) {
                host.ips = i.clone();
            }
        }

        Ok(hosts)
    }

    pub async fn delete(host_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM pam_hosts WHERE id = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(host_id)).await?;
        } else {
            DB::pg_execute(sql, &[&host_id]).await?;
        }

        Ok(())
    }

    pub async fn update(
        host_id: String,
        payload: PamHostUpdateRequest,
    ) -> Result<(), ErrorResponse> {
        let sql_update = r#"
UPDATE pam_hosts
SET hostname = $1, gid = $2, force_mfa = $3, local_password_only = $4, notes = $5
WHERE id = $6
"#;
        let sql_cleanup_1 = "DELETE FROM pam_hosts_ips WHERE host_id = $1";
        let sql_cleanup_2 = "DELETE FROM pam_hosts_aliases WHERE host_id = $1";

        let sql_ips = "INSERT INTO pam_hosts_ips (host_id, ip) VALUES ($1, $2)";
        let sql_aliases = "INSERT INTO pam_hosts_aliases (host_id, alias) VALUES ($1, $2)";

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(3 + payload.ips.len() + payload.aliases.len());

            txn.push((
                sql_update,
                params!(
                    payload.hostname,
                    payload.gid,
                    payload.force_mfa,
                    payload.local_password_only,
                    payload.notes,
                    host_id.clone()
                ),
            ));
            txn.push((sql_cleanup_1, params!(host_id.clone())));
            txn.push((sql_cleanup_2, params!(host_id.clone())));

            for ip in payload.ips {
                txn.push((sql_ips, params!(host_id.clone(), ip.to_string())));
            }
            for alias in payload.aliases {
                txn.push((sql_aliases, params!(host_id.clone(), alias)));
            }

            DB::hql().txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            DB::pg_txn_append(
                &txn,
                sql_update,
                &[
                    &payload.hostname,
                    &(payload.gid as i64),
                    &payload.force_mfa,
                    &payload.notes,
                    &host_id,
                ],
            )
            .await?;
            DB::pg_txn_append(&txn, sql_cleanup_1, &[&host_id]).await?;
            DB::pg_txn_append(&txn, sql_cleanup_2, &[&host_id]).await?;

            for ip in payload.ips {
                DB::pg_txn_append(&txn, sql_ips, &[&host_id, &ip.to_string()]).await?;
            }
            for alias in payload.aliases {
                DB::pg_txn_append(&txn, sql_aliases, &[&host_id, &alias]).await?;
            }

            txn.commit().await?;
        }

        Ok(())
    }

    pub async fn rotate_secret(host_id: String) -> Result<String, ErrorResponse> {
        let secret = secure_random_alnum(64);
        let enc = EncValue::encrypt(secret.as_bytes())?.into_bytes().to_vec();

        let sql = "UPDATE pam_hosts SET secret = $1 WHERE id = $2";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(enc, host_id)).await?;
        } else {
            DB::pg_execute(sql, &[&enc, &host_id]).await?;
        }

        Ok(secret)
    }
}

impl PamHost {
    pub async fn build_wheel_group_response(
        &self,
    ) -> Result<PamGroupMembersResponse, ErrorResponse> {
        let sql = r#"
SELECT name FROM pam_rel_groups_users pu
JOIN pam_users u ON pu.uid = u.id
WHERE pu.gid = $1 AND pu.wheel = $2
"#;

        let members = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!(self.gid, true))
                .await?
                .into_iter()
                .map(|mut r| r.get("name"))
                .collect::<Vec<String>>()
        } else {
            DB::pg_query_rows(sql, &[&(self.gid as i64), &true], 4)
                .await?
                .into_iter()
                .map(|r| r.get("name"))
                .collect::<Vec<String>>()
        };

        Ok(PamGroupMembersResponse {
            id: PAM_WHEEL_ID,
            name: PAM_WHEEL_NAME.to_string(),
            typ: PamGroupType::Immutable,
            members,
        })
    }

    #[inline]
    pub async fn is_user_in_group(&self, user: &PamUser) -> bool {
        let sql = "SELECT 1 FROM pam_rel_groups_users WHERE gid = $1 AND uid = $2";
        if is_hiqlite() {
            DB::hql()
                .query_raw_one(sql, params!(self.gid, user.id))
                .await
                .is_ok()
        } else {
            DB::pg_query_one_row(sql, &[&(self.gid as i64), &(user.id as i64)])
                .await
                .is_ok()
        }
    }

    #[inline]
    pub fn validate_secret(&self, secret: String) -> Result<(), ErrorResponse> {
        if self.secret == secret {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid secret",
            ))
        }
    }
}

impl From<PamHost> for PamHostSimpleResponse {
    fn from(h: PamHost) -> Self {
        Self {
            id: h.id,
            name: h.hostname,
            aliases: h.aliases,
            addresses: h.ips,
        }
    }
}

impl From<PamHost> for PamHostAccessResponse {
    fn from(h: PamHost) -> Self {
        Self {
            hostname: h.hostname,
            force_mfa: h.force_mfa,
            local_password_only: h.local_password_only,
            notes: h.notes,
            ips: h.ips,
            aliases: h.aliases,
        }
    }
}

impl From<PamHost> for PamHostDetailsResponse {
    fn from(h: PamHost) -> Self {
        Self {
            id: h.id,
            hostname: h.hostname,
            gid: h.gid,
            force_mfa: h.force_mfa,
            local_password_only: h.local_password_only,
            notes: h.notes,
            ips: h.ips,
            aliases: h.aliases,
        }
    }
}
