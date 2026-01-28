use crate::database::{Cache, DB};
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_APP_VERSION, RAUTHY_VERSION};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{deserialize, serialize};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};
use serde_json::value;
use std::time::Duration;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestAppVersion {
    pub timestamp: i64,
    pub latest_version: semver::Version,
    pub release_url: String,
}

impl LatestAppVersion {
    pub async fn find() -> Option<Self> {
        let client = DB::hql();

        if let Ok(Some(slf)) = client.get(Cache::App, IDX_APP_VERSION).await {
            return Some(slf);
        }

        let sql = "SELECT data FROM config WHERE id = 'latest_version'";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await.ok()
        } else {
            DB::pg_query_one_row(sql, &[])
                .await
                .ok()
                .map(|r| r.get::<_, Vec<u8>>("data"))
        };

        if let Some(data) = res {
            if let Ok(slf) = deserialize::<Self>(&data) {
                if let Err(err) = client
                    .put(Cache::App, IDX_APP_VERSION, &slf, CACHE_TTL_APP)
                    .await
                {
                    error!("Inserting LatestAppVersion into cache: {err:?}");
                }

                Some(slf)
            } else {
                None
            }
        } else {
            if let Err(err) = client
                .put(
                    Cache::App,
                    IDX_APP_VERSION,
                    &None::<Option<Self>>,
                    CACHE_TTL_APP,
                )
                .await
            {
                error!("Inserting LatestAppVersion into cache: {err:?}");
            }

            None
        }
    }

    pub async fn upsert(
        latest_version: semver::Version,
        release_url: String,
    ) -> Result<(), ErrorResponse> {
        let slf = Self {
            timestamp: Utc::now().timestamp(),
            latest_version,
            release_url,
        };
        let data = serialize(&slf)?;

        let sql = r#"
INSERT INTO config (id, data) VALUES ('latest_version', $1)
ON CONFLICT(id) DO UPDATE SET data = $1"#;

        if is_hiqlite() {
            DB::hql().execute(sql, params!(data)).await?;
        } else {
            DB::pg_execute(sql, &[&data]).await?;
        }

        DB::hql()
            .put(Cache::App, IDX_APP_VERSION, &slf, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    pub async fn lookup() -> Result<(semver::Version, String), ErrorResponse> {
        // it makes no sense to use the glopbal client here - no benefit from connection pooling
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .user_agent(format!("Rauthy v{RAUTHY_VERSION} App Version Checker"))
            .build()?;

        let res = client
            .get("https://api.github.com/repos/sebadob/rauthy/releases/latest")
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await;

        match res {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        format!("Error fetching latest Rauthy App Version:\n{text}"),
                    ));
                }

                let json = resp.json::<value::Value>().await.map_err(|err| {
                    ErrorResponse::new(
                        ErrorResponseType::Internal,
                        format!("decoding GitHub response JSON: {err:?}"),
                    )
                })?;

                let tag_name = if let Some(tag_name) = json.get("tag_name") {
                    let tag_name = tag_name.as_str().unwrap_or_default();
                    let (_, v) = tag_name.split_once('v').unwrap_or(("", RAUTHY_VERSION));
                    semver::Version::parse(v).map_err(|err| {
                        ErrorResponse::new(
                            ErrorResponseType::Internal,
                            format!("parsing remote Rauthy App version. {err:?}"),
                        )
                    })
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "Could not find 'tag_name' in Rauthy App Version lookup response",
                    ))
                }?;

                let html_url = json
                    .get("html_url")
                    .map(|v| v.as_str().unwrap_or_default().to_string())
                    .unwrap_or_else(|| {
                        format!("https://github.com/sebadob/rauthy/releases/tag/v{tag_name}")
                    });

                Ok((tag_name, html_url))
            }

            Err(err) => Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Error fetching the latest Rauthy App Version: {err:?}"),
            )),
        }
    }
}
