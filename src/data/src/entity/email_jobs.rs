use crate::database::DB;
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use hiqlite::Row;
use hiqlite_macros::params;
use rauthy_api_types::email_jobs::{EmailJobFilterType, EmailJobRequest, EmailJobResponse};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use std::fmt::{Display, Formatter};
use std::ops::Sub;
use tracing::error;

#[derive(Debug)]
pub struct EmailJob {
    pub id: i64,
    pub scheduled: Option<i64>,
    pub status: EmailJobStatus,
    pub updated: i64,
    pub last_user_ts: i64,
    pub filter: EmailJobFilter,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub enum EmailJobFilter {
    None,
    InGroup(String),
    NotInGroup(String),
    HasRole(String),
    HasNotRole(String),
}

#[derive(Debug, PartialEq)]
pub enum EmailJobStatus {
    Open,
    Finished,
    Canceled,
}

impl EmailJobStatus {
    fn value(&self) -> i16 {
        match self {
            EmailJobStatus::Open => 0,
            EmailJobStatus::Finished => 1,
            EmailJobStatus::Canceled => 2,
        }
    }
}

impl From<i16> for EmailJobStatus {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::Open,
            1 => Self::Finished,
            2 => Self::Canceled,
            i => {
                panic!("Invalid EmailJobStatus in DB: {i}")
            }
        }
    }
}

impl From<&str> for EmailJobFilter {
    fn from(s: &str) -> Self {
        if let Some(group) = s.strip_prefix("group_") {
            Self::InGroup(group.to_string())
        } else if let Some(group) = s.strip_prefix("not_group_") {
            Self::NotInGroup(group.to_string())
        } else if let Some(role) = s.strip_prefix("role_") {
            Self::HasRole(role.to_string())
        } else if let Some(role) = s.strip_prefix("not_role_") {
            Self::HasNotRole(role.to_string())
        } else if s != "none" {
            error!("Invalid EmailJobFilter value in DB: {s}");
            Self::None
        } else {
            Self::None
        }
    }
}

impl Display for EmailJobFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailJobFilter::None => {
                write!(f, "none")
            }
            EmailJobFilter::InGroup(s) => {
                write!(f, "group_{s}")
            }
            EmailJobFilter::NotInGroup(s) => {
                write!(f, "not_group_{s}")
            }
            EmailJobFilter::HasRole(s) => {
                write!(f, "role_{s}")
            }
            EmailJobFilter::HasNotRole(s) => {
                write!(f, "not_role_{s}")
            }
        }
    }
}

impl From<hiqlite::Row<'_>> for EmailJob {
    fn from(mut row: Row<'_>) -> Self {
        // This downcasting is "safe" because Rauthy controls the input to the DB.
        let status = EmailJobStatus::from(row.get::<i64>("status") as i16);
        let filter = EmailJobFilter::from(row.get::<String>("filter").as_str());

        Self {
            id: row.get("id"),
            scheduled: row.get("scheduled"),
            status,
            updated: row.get("updated"),
            last_user_ts: row.get("last_user_ts"),
            filter,
            subject: row.get("subject"),
            body: row.get("body"),
        }
    }
}

impl From<tokio_postgres::Row> for EmailJob {
    fn from(row: tokio_postgres::Row) -> Self {
        let status = EmailJobStatus::from(row.get::<_, i16>("status"));
        let filter = EmailJobFilter::from(row.get::<_, String>("filter").as_str());

        Self {
            id: row.get("id"),
            scheduled: row.get("scheduled"),
            status,
            updated: row.get("updated"),
            last_user_ts: row.get("last_user_ts"),
            filter,
            subject: row.get("subject"),
            body: row.get("body"),
        }
    }
}

impl EmailJob {
    pub async fn insert(payload: EmailJobRequest) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let status = EmailJobStatus::Open.value();
        let filter = if let Some(value) = payload.filter_value {
            match payload.filter_type {
                EmailJobFilterType::None => EmailJobFilter::None,
                EmailJobFilterType::InGroup => EmailJobFilter::InGroup(value),
                EmailJobFilterType::NotInGroup => EmailJobFilter::NotInGroup(value),
                EmailJobFilterType::HasRole => EmailJobFilter::HasRole(value),
                EmailJobFilterType::HasNotRole => EmailJobFilter::HasNotRole(value),
            }
        } else {
            EmailJobFilter::None
        }
        .to_string();

        let sql = r#"
INSERT INTO email_jobs (id, scheduled, status, updated, filter, subject, body)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        now,
                        payload.scheduled,
                        status,
                        now,
                        filter,
                        payload.subject,
                        payload.body
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &now,
                    &payload.scheduled,
                    &status,
                    &now,
                    &filter,
                    &payload.subject,
                    &payload.body,
                ],
            )
            .await?;
        }

        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM email_jobs";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        Ok(res)
    }

    /// Finds jobs that have not been updated in a long time. This can happen for very long-running
    /// jobs, when for instance the executing instance was restarted or something like that.
    ///
    /// In HA deployments, only the current leader should look for orphaned jobs and pick them
    /// up to avoid duplicate emails because of possible race conditions.
    pub async fn find_orphaned() -> Result<Vec<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM email_jobs
WHERE status = 0 AND updated < $2"#;

        let threshold = Utc::now()
            .sub(chrono::Duration::seconds(
                RauthyConfig::get().vars.email.jobs.orphaned_seconds as i64,
            ))
            .timestamp();

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(threshold)).await?
        } else {
            DB::pg_query(sql, &[&threshold], 0).await?
        };

        Ok(res)
    }

    /// Updates:
    /// - scheduled
    /// - status
    /// - updated
    /// - last_user_ts
    pub async fn save(&mut self) -> Result<(), ErrorResponse> {
        let sql = r#"
UPDATE email_jobs
SET scheduled = $1, status = $2, updated = $3, last_user_ts = $4"#;

        self.updated = Utc::now().timestamp();
        let status = self.status.value();
        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(self.scheduled, status, self.updated, self.last_user_ts),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&self.scheduled, &status, &self.updated, &self.last_user_ts],
            )
            .await?;
        }

        Ok(())
    }
}

impl EmailJob {
    pub async fn spawn_task(self) -> Result<(), ErrorResponse> {
        todo!()
    }
}

impl From<EmailJobStatus> for rauthy_api_types::email_jobs::EmailJobStatus {
    fn from(s: EmailJobStatus) -> Self {
        match s {
            EmailJobStatus::Open => rauthy_api_types::email_jobs::EmailJobStatus::Open,
            EmailJobStatus::Finished => rauthy_api_types::email_jobs::EmailJobStatus::Finished,
            EmailJobStatus::Canceled => rauthy_api_types::email_jobs::EmailJobStatus::Canceled,
        }
    }
}

impl From<EmailJob> for EmailJobResponse {
    fn from(v: EmailJob) -> Self {
        let (filter_type, filter_value) = match v.filter {
            EmailJobFilter::None => (EmailJobFilterType::None, None),
            EmailJobFilter::InGroup(s) => (EmailJobFilterType::InGroup, Some(s)),
            EmailJobFilter::NotInGroup(s) => (EmailJobFilterType::NotInGroup, Some(s)),
            EmailJobFilter::HasRole(s) => (EmailJobFilterType::HasRole, Some(s)),
            EmailJobFilter::HasNotRole(s) => (EmailJobFilterType::HasNotRole, Some(s)),
        };

        Self {
            id: v.id,
            scheduled: v.scheduled,
            status: v.status.into(),
            updated: v.updated,
            last_user_ts: v.last_user_ts,
            filter_type,
            filter_value,
            subject: v.subject,
            body: v.body,
        }
    }
}
