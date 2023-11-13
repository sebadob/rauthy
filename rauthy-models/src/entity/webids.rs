use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_12HR, PUB_URL_WITH_SCHEME};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use rio_api::formatter::TriplesFormatter;
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleFormatter};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub custom_triples: Option<String>,
    pub expose_email: bool,
}

impl WebId {
    #[inline]
    pub fn resolve_webid_uri(user_id: &str) -> String {
        format!("{}/auth/webid/{}/profile#me", *PUB_URL_WITH_SCHEME, user_id)
    }

    #[inline]
    pub fn resolve_webid_card_uri(user_id: &str) -> String {
        format!("{}/auth/webid/{}/profile", *PUB_URL_WITH_SCHEME, user_id)
    }

    /// Returns the WebId from the database, if it exists, and a default otherwise.
    pub async fn find(data: &web::Data<AppState>, user_id: String) -> Result<Self, ErrorResponse> {
        if let Some(web_id) = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
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
            CACHE_NAME_12HR.to_string(),
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
            ON CONFLICT(user_id) DO UPDATE SET data = $2, expose_email = $3"#,
            web_id.user_id,
            web_id.custom_triples,
            web_id.expose_email,
        );
        q.execute(&data.db).await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
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

    #[inline]
    fn clean_up_triple_data(custom_triples: &str) -> String {
        // We need to clean up things like linefeeds before the triples parse will be happy
        let mut cleaned = custom_triples.replace(['\n', ';'], "");

        // Additionally, the parse complains if there is no '.' at the end
        if !cleaned.ends_with('.') {
            cleaned.push('.');
        }

        // TODO for some reason, we cannot input data into the parser with ';', which separates
        // these triples in the final format. How should we handle this?
        // It seems we cannot put in multiple triples at once, even though the `parse_all()`
        // is being called

        cleaned
    }

    pub fn try_fmt_triples(
        custom_triples: &str,
        formatter: &mut TurtleFormatter<Vec<u8>>,
    ) -> Result<(), ErrorResponse> {
        // The parser complains about a lof of things.
        // To have a good UX when editing the custom data in the UI, we should not save the
        // data cleaned up, but actually return it to the user later again like it was written,
        // with all linebreaks, spaces, and so on.
        // -> cleanup dynamically here each time before actually parsing it
        let triples = Self::clean_up_triple_data(custom_triples);

        NTriplesParser::new(triples.as_bytes()).parse_all(&mut |t| {
            tracing::debug!("\n\n triple in parse_all: {:?}\n", t);
            formatter.format(&t).map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Cannot format custom Triple {}: {:?}", t, err),
                )
            })
        })?;

        Ok(())
    }

    pub fn validate_custom_triples(&self) -> Result<(), ErrorResponse> {
        let mut formatter = TurtleFormatter::new(Vec::<u8>::new());
        if let Some(triples) = &self.custom_triples {
            Self::try_fmt_triples(triples, &mut formatter)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::webids::WebId;

    // Tests the validation for a String that comes from the UI account page in the end.
    // Whatever input should work in the UI must be passing this test.
    // TODO expand this test
    #[test]
    fn test_custom_triple_validation() {
        let mut web_id = WebId {
            user_id: "SomeId123".to_string(),
            custom_triples: None,
            expose_email: false,
        };

        // success without any triples of course
        assert!(web_id.validate_custom_triples().is_ok());

        // TODO add more tests here which make sense
        web_id.custom_triples = Some(
            r#"
<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me>
<http://www.w3.org/ns/solid/terms#oidcIssuer>
<http://localhost:8080/auth/v1>
"#
            .to_string(),
        );
        assert!(web_id.validate_custom_triples().is_ok());

        // TODO for some reason, we cannot input data into the parser with ';', which separates
        // these triples in the final format. How should we handle this?
        // It seems we cannot put in multiple triples at once, even though the `parse_all()`
        // is being called
        web_id.custom_triples = Some(
            r#"
<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me>
 <http://www.w3.org/ns/solid/terms#oidcIssuer>
<http://localhost:8080/auth/v1>;
"#
            .to_string(),
        );
        // helps with debugging the test case
        if let Err(err) = web_id.validate_custom_triples() {
            eprintln!("{}", err.message);
        }
        // TODO assert!(web_id.validate_custom_triples().is_ok());
        assert!(web_id.validate_custom_triples().is_ok());
    }
}
