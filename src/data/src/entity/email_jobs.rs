use crate::database::DB;
use crate::email;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use hiqlite::Row;
use hiqlite_macros::params;
use rauthy_api_types::email_jobs::{EmailJobFilterType, EmailJobRequest, EmailJobResponse};
use rauthy_common::{is_hiqlite, markdown};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct EmailJob {
    pub id: i64,
    pub scheduled: Option<i64>,
    pub status: EmailJobStatus,
    pub updated: i64,
    pub last_user_ts: Option<String>,
    pub filter: EmailJobFilter,
    pub content_type: EmailContentType,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub enum EmailContentType {
    Text,
    Markdown,
    Html,
}

impl From<&str> for EmailContentType {
    fn from(s: &str) -> Self {
        match s {
            "markdown" => Self::Markdown,
            "html" => Self::Html,
            _ => Self::Text,
        }
    }
}

impl Display for EmailContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailContentType::Text => write!(f, "text"),
            EmailContentType::Markdown => write!(f, "markdown"),
            EmailContentType::Html => write!(f, "html"),
        }
    }
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
        let content_type = EmailContentType::from(row.get::<String>("content_type").as_str());

        Self {
            id: row.get("id"),
            scheduled: row.get("scheduled"),
            status,
            updated: row.get("updated"),
            last_user_ts: row.get("last_user_ts"),
            filter,
            content_type,
            subject: row.get("subject"),
            body: row.get("body"),
        }
    }
}

impl From<tokio_postgres::Row> for EmailJob {
    fn from(row: tokio_postgres::Row) -> Self {
        let status = EmailJobStatus::from(row.get::<_, i16>("status"));
        let filter = EmailJobFilter::from(row.get::<_, String>("filter").as_str());
        let content_type = EmailContentType::from(row.get::<_, String>("content_type").as_str());

        Self {
            id: row.get("id"),
            scheduled: row.get("scheduled"),
            status,
            updated: row.get("updated"),
            last_user_ts: row.get("last_user_ts"),
            filter,
            content_type,
            subject: row.get("subject"),
            body: row.get("body"),
        }
    }
}

impl EmailJob {
    pub async fn cancel(&mut self) -> Result<(), ErrorResponse> {
        let sql = "UPDATE email_jobs SET status = $1 WHERE id = $2";

        let status = EmailJobStatus::Canceled.value();
        if is_hiqlite() {
            DB::hql().execute(sql, params!(status, self.id)).await?;
        } else {
            DB::pg_execute(sql, &[&status, &self.id]).await?;
        }
        self.status = EmailJobStatus::Canceled;

        Ok(())
    }

    /// Expects body to be already sanitized if `is_html`!
    pub async fn insert(payload: EmailJobRequest) -> Result<Self, ErrorResponse> {
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
        let content_type = EmailContentType::from(payload.content_type).to_string();

        let sql = r#"
INSERT INTO email_jobs (id, scheduled, status, updated, filter, content_type, subject, body)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING *"#;

        let slf = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(
                    sql,
                    params!(
                        now,
                        payload.scheduled,
                        status,
                        now,
                        filter,
                        content_type,
                        payload.subject,
                        payload.body
                    ),
                )
                .await?
        } else {
            DB::pg_query_one(
                sql,
                &[
                    &now,
                    &payload.scheduled,
                    &status,
                    &now,
                    &filter,
                    &content_type,
                    &payload.subject,
                    &payload.body,
                ],
            )
            .await?
        };

        Ok(slf)
    }

    pub async fn find(id: i64) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM email_jobs WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM email_jobs ORDER BY id DESC";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        Ok(res)
    }

    /// Finds jobs that have not been updated in a long time, or are scheduled for "now".
    ///
    /// In HA deployments, only the current leader should look for orphaned jobs and pick them
    /// up to avoid duplicate emails because of possible race conditions.
    pub async fn find_orphaned_or_schedule() -> Result<Vec<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM email_jobs
WHERE status = 0 AND updated < $2 AND (scheduled IS NULL OR scheduled <= $1)"#;

        let now = Utc::now().timestamp();
        let threshold = now - RauthyConfig::get().vars.email.jobs.orphaned_seconds as i64;

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(now, threshold)).await?
        } else {
            DB::pg_query(sql, &[&now, &threshold], 0).await?
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
SET scheduled = $1, status = $2, updated = $3, last_user_ts = $4
WHERE id = $5"#;

        self.updated = Utc::now().timestamp();
        let status = self.status.value();
        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.scheduled,
                        status,
                        self.updated,
                        &self.last_user_ts,
                        self.id
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.scheduled,
                    &status,
                    &self.updated,
                    &self.last_user_ts,
                    &self.id,
                ],
            )
            .await?;
        }

        Ok(())
    }
}

impl EmailJob {
    async fn render_html_content(&self) -> Result<Option<String>, ErrorResponse> {
        let body = match self.content_type {
            EmailContentType::Text => {
                return Ok(None);
            }
            EmailContentType::Markdown => {
                Cow::from(markdown::render_sanitized_markdown(&self.body))
            }
            EmailContentType::Html => Cow::from(self.body.to_string()),
        };

        Ok(Some(email::custom::build_custom_html_email(&body).await?))
    }

    pub fn spawn_task(self) {
        info!("Spawning EmailJob background task {}", self.id);

        tokio::task::spawn(async move {
            if let Err(err) = self.task_execute().await {
                error!("Error during EmailJob Task: {}", err.message);
            }
        });
    }

    #[inline]
    async fn task_execute(mut self) -> Result<(), ErrorResponse> {
        let (batch_size, delay) = {
            let vars = &RauthyConfig::get().vars.email.jobs;
            (
                vars.batch_size,
                Duration::from_millis(vars.batch_delay_ms as u64),
            )
        };
        let mut delay_inner_ms = 10;

        let (text, html) = match self.content_type {
            EmailContentType::Text => (Some(self.body.clone()), None),
            EmailContentType::Markdown => {
                (Some(self.body.clone()), self.render_html_content().await?)
            }
            EmailContentType::Html => (None, self.render_html_content().await?),
        };

        let mut cancel_check_ts = Utc::now().timestamp();

        loop {
            {
                let now = Utc::now().timestamp();
                if now + 30 > cancel_check_ts {
                    let slf = Self::find(self.id).await?;
                    if slf.status == EmailJobStatus::Canceled {
                        warn!("EmailJob Task has been canceled externally - exiting");
                        return Ok(());
                    }
                    cancel_check_ts = now;
                }
            }

            if self.status != EmailJobStatus::Open {
                info!(
                    "EmailJob Task {} finished with status {:?}",
                    self.id, self.status
                );
                return Ok(());
            }

            let users = {
                let (email, ts) = self.last_user_ts();
                User::find_batch(email, ts, batch_size).await?
            };
            if users.is_empty() {
                self.status = EmailJobStatus::Finished;
                self.save().await?;
                continue;
            }

            for user in users {
                // TODO remove after debugging
                warn!("EmailJob Task for User: {:?}", user);

                match &self.filter {
                    EmailJobFilter::None => {}
                    EmailJobFilter::InGroup(s) => {
                        if !user.get_groups().contains(s) {
                            self.set_last_user_ts(&user.email, user.created_at);
                            continue;
                        }
                    }
                    EmailJobFilter::NotInGroup(s) => {
                        if user.get_groups().contains(s) {
                            self.set_last_user_ts(&user.email, user.created_at);
                            continue;
                        }
                    }
                    EmailJobFilter::HasRole(s) => {
                        if !user.get_roles().contains(s) {
                            self.set_last_user_ts(&user.email, user.created_at);
                            continue;
                        }
                    }
                    EmailJobFilter::HasNotRole(s) => {
                        if user.get_roles().contains(s) {
                            self.set_last_user_ts(&user.email, user.created_at);
                            continue;
                        }
                    }
                }

                // catch a send timeout and increase delay timing
                for i in 0..5 {
                    match email::custom::send_custom(
                        &user,
                        &self.subject,
                        text.clone(),
                        html.clone(),
                    )
                    .await
                    {
                        Ok(_) => {
                            // TODO change to `trace` level after initial debugging
                            debug!("E-Mail sent successfully to {}", user.email);
                            self.set_last_user_ts(&user.email, user.created_at);
                            break;
                        }
                        Err(err) => {
                            if i >= 4 {
                                self.save().await?;
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::Timeout,
                                    "Reached E-Mail send timeout - retries exceeded - \
                                    exiting job",
                                ));
                            }

                            let delay_new = delay_inner_ms * 3;
                            error!(
                                ?err,
                                "Reached E-Mail send timeout after {i} retries, increasing batch \
                                delay_inner_ms from {delay_inner_ms} to {delay_new}"
                            );
                            delay_inner_ms = delay_new;

                            tokio::time::sleep(Duration::from_millis(delay_inner_ms)).await;
                        }
                    }
                }

                // Email sending happens on a dedicated thread and if we would not sleep at least
                // a short amount of time inside the batch, the channel could get filled up pretty
                // quickly with slow servers.
                tokio::time::sleep(Duration::from_millis(delay_inner_ms)).await;
            }

            self.save().await?;
            tokio::time::sleep(delay).await;
        }
    }

    #[inline]
    fn last_user_ts(&self) -> (&str, i64) {
        if let Some(last_user_ts) = &self.last_user_ts {
            let (ts, email) = last_user_ts.split_at(10);
            (email, ts.parse().unwrap())
        } else {
            ("", 0)
        }
    }

    #[inline]
    fn set_last_user_ts(&mut self, email: &str, created_at: i64) {
        self.last_user_ts = Some(format!("{created_at}{email}"));
    }
}

impl From<EmailJobStatus> for rauthy_api_types::email_jobs::EmailJobStatus {
    fn from(s: EmailJobStatus) -> Self {
        match s {
            EmailJobStatus::Open => Self::Open,
            EmailJobStatus::Finished => Self::Finished,
            EmailJobStatus::Canceled => Self::Canceled,
        }
    }
}

impl From<EmailContentType> for rauthy_api_types::email_jobs::EmailContentType {
    fn from(value: EmailContentType) -> Self {
        match value {
            EmailContentType::Text => Self::Text,
            EmailContentType::Markdown => Self::Markdown,
            EmailContentType::Html => Self::HTML,
        }
    }
}

impl From<rauthy_api_types::email_jobs::EmailContentType> for EmailContentType {
    fn from(value: rauthy_api_types::email_jobs::EmailContentType) -> Self {
        match value {
            rauthy_api_types::email_jobs::EmailContentType::Text => Self::Text,
            rauthy_api_types::email_jobs::EmailContentType::Markdown => Self::Markdown,
            rauthy_api_types::email_jobs::EmailContentType::HTML => Self::Html,
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
        let content_type = match v.content_type {
            EmailContentType::Text => rauthy_api_types::email_jobs::EmailContentType::Text,
            EmailContentType::Markdown => rauthy_api_types::email_jobs::EmailContentType::Markdown,
            EmailContentType::Html => rauthy_api_types::email_jobs::EmailContentType::HTML,
        };

        Self {
            id: v.id,
            scheduled: v.scheduled,
            status: v.status.into(),
            updated: v.updated,
            last_user_ts: v.last_user_ts,
            filter_type,
            filter_value,
            content_type,
            subject: v.subject,
            body: v.body,
        }
    }
}
