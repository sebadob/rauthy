use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_USERS, PUB_URL_WITH_SCHEME};
use rauthy_common::error_response::ErrorResponse;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use rio_api::formatter::TriplesFormatter;
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesFormatter, NTriplesParser, TurtleFormatter, TurtleParser};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct WebId {
    pub user_id: String,
    pub expose_email: bool,
    pub custom_triples: Option<String>,
}

impl WebId {
    #[inline]
    pub fn resolve_webid_uri(user_id: &str) -> String {
        format!("{}/auth/{}/profile#me", *PUB_URL_WITH_SCHEME, user_id)
    }

    #[inline]
    pub fn resolve_webid_card_uri(user_id: &str) -> String {
        format!("{}/auth/{}/profile", *PUB_URL_WITH_SCHEME, user_id)
    }

    /// Returns the WebId from the database, if it exists, and a default otherwise.
    pub async fn find(data: &web::Data<AppState>, user_id: String) -> Result<Self, ErrorResponse> {
        if let Some(web_id) = cache_get!(
            Self,
            CACHE_NAME_USERS.to_string(),
            Self::cache_idx(&user_id),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(web_id);
        }

        let slf = match query_as!(Self, "SELECT * FROM webids WHERE user_id = $1", user_id)
            .fetch_one(&data.db)
            .await
        {
            Ok(row) => row,
            Err(_) => {
                // if there is no custom data available, just return defaults
                Self {
                    user_id,
                    custom_triples: None,
                    expose_email: false,
                }
            }
        };

        cache_insert(
            CACHE_NAME_USERS.to_string(),
            Self::cache_idx(&slf.user_id),
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Leader,
        )
        .await?;

        Ok(slf)
    }

    pub async fn upsert(data: &web::Data<AppState>, web_id: WebId) -> Result<(), ErrorResponse> {
        #[cfg(feature = "sqlite")]
        let q = query!(
            "INSERT OR REPLACE INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)",
            web_id.user_id,
            web_id.custom_triples,
            web_id.expose_email,
        );
        #[cfg(not(feature = "sqlite"))]
        let q = query!(
            r#"INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)
            ON CONFLICT(user_id) DO UPDATE SET custom_triples = $2, expose_email = $3"#,
            web_id.user_id,
            web_id.custom_triples,
            web_id.expose_email,
        );
        q.execute(&data.db).await?;

        cache_insert(
            CACHE_NAME_USERS.to_string(),
            Self::cache_idx(&web_id.user_id),
            &data.caches.ha_cache_config,
            &web_id,
            AckLevel::Leader,
        )
        .await?;

        Ok(())
    }
}

impl WebId {
    #[inline]
    fn cache_idx(user_id: &str) -> String {
        format!("web_id_{}", user_id)
    }

    pub fn try_new(
        user_id: String,
        custom_data_turtle: Option<&str>,
        expose_email: bool,
    ) -> Result<Self, ErrorResponse> {
        let custom_triples = if let Some(data) = custom_data_turtle {
            let mut ttl_parser = TurtleParser::new(
                data.as_bytes(),
                Some(Self::resolve_webid_card_uri(&user_id).parse()?),
            );
            let mut ntriples_fmt = NTriplesFormatter::new(Vec::<u8>::new());
            ttl_parser.parse_all(&mut |t| ntriples_fmt.format(&t))?;

            // as long as the formatter itself is not buggy, this String conversion should never panic
            Some(String::from_utf8(ntriples_fmt.finish().unwrap()).expect("Must be valid string."))
        } else {
            None
        };

        Ok(Self {
            user_id,
            custom_triples,
            expose_email,
        })
    }

    pub fn fmt_custom_triples_to_ttl(
        &self,
        formatter: &mut TurtleFormatter<Vec<u8>>,
    ) -> Result<(), ErrorResponse> {
        if let Some(custom_triples) = self.custom_triples.as_ref() {
            NTriplesParser::new(custom_triples.as_bytes()).parse_all(&mut |t| {
                formatter.format(&t)?;
                Ok(())
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::env;

    use crate::entity::webids::WebId;

    // Tests the validation for a String that comes from the UI account page in the end.
    // Whatever input should work in the UI must be passing this test.
    // TODO expand this test -> still only accepts a single triple and not multiple ones
    #[rstest]
    #[case(None)]
    #[case(Some(
        r#"
<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me>
<http://www.w3.org/ns/solid/terms#oidcIssuer>
<http://localhost:8080/auth/v1> .
"#
    ))]
    #[case(Some(
        r#"
<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me>
<http://www.w3.org/ns/solid/terms#oidcIssuer>
<http://localhost:8080/auth/v1>; .
"#
    ))]
    fn test_custom_triple_validation(#[case] custom_data_turtle: Option<&str>) {
        env::set_var("PUB_URL", "localhost:8081".to_string());
        let user_id = "SomeId123".to_owned();
        let expose_email = false;

        assert!(WebId::try_new(user_id, custom_data_turtle, expose_email).is_ok());
    }
}
