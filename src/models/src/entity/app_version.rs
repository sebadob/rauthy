use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use actix_web::web;
use chrono::Utc;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_APP_VERSION, RAUTHY_VERSION};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};
use serde_json::value;
use sqlx::query;
use std::time::Duration;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestAppVersion {
    pub timestamp: i64,
    pub latest_version: semver::Version,
    pub release_url: String,
}

impl LatestAppVersion {
    pub async fn find(app_state: &web::Data<AppState>) -> Option<Self> {
        let client = DB::client();

        if let Ok(Some(slf)) = client.get(Cache::App, IDX_APP_VERSION).await {
            return Some(slf);
        }

        let res = query!("select data from config where id = 'latest_version'")
            .fetch_optional(&app_state.db)
            .await
            .ok()?;

        match res {
            Some(res) => {
                let data = res
                    .data
                    .expect("to get 'data' back from the AppVersion query");
                if let Ok(slf) = bincode::deserialize::<Self>(&data) {
                    if let Err(err) = client
                        .put(Cache::App, IDX_APP_VERSION, &slf, CACHE_TTL_APP)
                        .await
                    {
                        error!("Inserting LatestAppVersion into cache: {:?}", err);
                    }

                    Some(slf)
                } else {
                    None
                }
            }
            None => {
                if let Err(err) = client
                    .put(
                        Cache::App,
                        IDX_APP_VERSION,
                        &None::<Option<Self>>,
                        CACHE_TTL_APP,
                    )
                    .await
                {
                    error!("Inserting LatestAppVersion into cache: {:?}", err);
                }

                None
            }
        }
    }

    pub async fn upsert(
        app_state: &web::Data<AppState>,
        latest_version: semver::Version,
        release_url: String,
    ) -> Result<(), ErrorResponse> {
        let slf = Self {
            timestamp: Utc::now().timestamp(),
            latest_version,
            release_url,
        };
        let data = bincode::serialize(&slf)?;

        #[cfg(not(feature = "postgres"))]
        let q = query!(
            "insert or replace into config (id, data) values ('latest_version', $1)",
            data,
        );
        #[cfg(feature = "postgres")]
        let q = query!(
            r#"insert into config (id, data) values ('latest_version', $1)
            on conflict(id) do update set data = $1"#,
            data,
        );
        q.execute(&app_state.db).await?;

        DB::client()
            .put(Cache::App, IDX_APP_VERSION, &slf, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    pub async fn lookup() -> Result<(semver::Version, String), ErrorResponse> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .user_agent(format!("Rauthy v{} App Version Checker", RAUTHY_VERSION))
            .build()
            .unwrap();

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
                        format!("Error fetching latest Rauthy App Version:\n{}", text),
                    ));
                }

                let json = resp.json::<value::Value>().await.map_err(|err| {
                    ErrorResponse::new(
                        ErrorResponseType::Internal,
                        format!("decoding Github response JSON: {:?}", err),
                    )
                })?;

                let tag_name = if let Some(tag_name) = json.get("tag_name") {
                    let tag_name = tag_name.as_str().unwrap_or_default();
                    let (_, v) = tag_name.split_once('v').unwrap_or(("", RAUTHY_VERSION));
                    semver::Version::parse(v).map_err(|err| {
                        ErrorResponse::new(
                            ErrorResponseType::Internal,
                            format!("parsing remote Rauthy App version. {:?}", err),
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
                        format!(
                            "https://github.com/sebadob/rauthy/releases/tag/v{}",
                            tag_name
                        )
                    });

                Ok((tag_name, html_url))
            }

            Err(err) => Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Error fetching the latest Rauthy App Version: {:?}", err),
            )),
        }
    }
}
