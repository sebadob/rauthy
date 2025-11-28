use hiqlite::Row;
use rauthy_api_types::email_jobs::{EmailJobFilterType, EmailJobRequest, EmailJobResponse};
use rauthy_error::ErrorResponse;
use std::fmt::{Display, Formatter};
use tracing::error;

#[derive(Debug)]
pub struct EmailJob {
    pub id: i64,
    pub scheduled: Option<i64>,
    pub status: EmailJobStatus,
    pub updated: bool,
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
    Open = 0,
    Finished,
    Canceled,
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
        todo!()
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        todo!()
    }

    /// Finds jobs that have not been updated in a long time. This can happen for very long-running
    /// jobs, when for instance the executing instance was restarted or something like that.
    ///
    /// In HA deployments, only the current leader should look for orphaned jobs and pick them
    /// up to avoid duplicate emails because of possible race conditions.
    pub async fn find_orphaned() -> Result<Vec<Self>, ErrorResponse> {
        todo!()
    }

    pub async fn set_status(id: i64, status: EmailJobStatus) -> Result<(), ErrorResponse> {
        todo!()
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        todo!()
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
