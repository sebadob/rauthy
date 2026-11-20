use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use hiqlite::macros::params;
use oxrdf::vocab::rdf;
use oxrdf::{LiteralRef, NamedNodeRef, TripleRef};
use oxttl::{NTriplesParser, NTriplesSerializer, TurtleParser, TurtleSerializer};
use rauthy_api_types::users::WebIdResponse;
use rauthy_common::constants::CACHE_TTL_USER;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// The vocabulary a WebID card is built from. `rdf:type` comes from `oxrdf::vocab::rdf`.
const FOAF_PERSONAL_PROFILE_DOCUMENT: &str = "http://xmlns.com/foaf/0.1/PersonalProfileDocument";
const FOAF_PRIMARY_TOPIC: &str = "http://xmlns.com/foaf/0.1/primaryTopic";
const FOAF_PERSON: &str = "http://xmlns.com/foaf/0.1/Person";
const FOAF_GIVEN_NAME: &str = "http://xmlns.com/foaf/0.1/givenname";
const FOAF_FAMILY_NAME: &str = "http://xmlns.com/foaf/0.1/family_name";
const FOAF_MBOX: &str = "http://xmlns.com/foaf/0.1/mbox";
const SOLID_OIDC_ISSUER: &str = "http://www.w3.org/ns/solid/terms#oidcIssuer";

#[derive(Debug, Clone, Serialize, Deserialize, FromPgRow, utoipa::ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub expose_email: bool,
    pub custom_triples: Option<String>,
}

impl WebId {
    #[inline]
    pub fn resolve_webid_uri(user_id: &str) -> String {
        format!(
            "{}/auth/{user_id}/profile#me",
            RauthyConfig::get().pub_url_with_scheme
        )
    }

    #[inline]
    pub fn resolve_webid_card_uri(user_id: &str, pub_url: &str) -> String {
        format!("{pub_url}/auth/{user_id}/profile")
    }

    /// Returns the WebId from the database, if it exists, and a default otherwise.
    pub async fn find(user_id: String) -> Result<Self, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::User, Self::cache_idx(&user_id)).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM webids WHERE user_id = $1";
        let slf = if is_hiqlite() {
            client
                .query_as_one(sql, params!(&user_id))
                .await
                .unwrap_or(Self {
                    user_id,
                    custom_triples: None,
                    expose_email: false,
                })
        } else {
            DB::pg_query_one(sql, &[&user_id]).await?
        };

        client
            .put(
                Cache::User,
                Self::cache_idx(&slf.user_id),
                &slf,
                CACHE_TTL_USER,
            )
            .await?;

        Ok(slf)
    }

    pub async fn upsert(web_id: WebId) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO webids (user_id, custom_triples, expose_email)
VALUES ($1, $2, $3)
ON CONFLICT(user_id) DO UPDATE
SET custom_triples = $2, expose_email = $3"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(&web_id.user_id, &web_id.custom_triples, web_id.expose_email),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &web_id.user_id,
                    &web_id.custom_triples,
                    &web_id.expose_email,
                ],
            )
            .await?;
        }

        DB::hql()
            .put(
                Cache::User,
                Self::cache_idx(&web_id.user_id),
                &web_id,
                CACHE_TTL_USER,
            )
            .await?;

        Ok(())
    }
}

impl WebId {
    #[inline]
    fn cache_idx(user_id: &str) -> String {
        format!("web_id_{user_id}")
    }

    pub fn try_new(
        user_id: String,
        custom_data_turtle: Option<&str>,
        expose_email: bool,
        pub_url: &str,
    ) -> Result<Self, ErrorResponse> {
        let custom_triples = if let Some(data) = custom_data_turtle {
            // Relative IRIs inside the user's input are resolved against this WebID card.
            let parser = TurtleParser::new()
                .with_base_iri(Self::resolve_webid_card_uri(&user_id, pub_url))?;
            let mut ntriples_fmt = NTriplesSerializer::new().for_writer(Vec::<u8>::new());
            for triple in parser.for_slice(data.as_bytes()) {
                ntriples_fmt.serialize_triple(triple?.as_ref())?;
            }

            // as long as the formatter itself is not buggy, this String conversion should never panic
            Some(String::from_utf8(ntriples_fmt.finish()).expect("Must be valid string."))
        } else {
            None
        };

        Ok(Self {
            user_id,
            custom_triples,
            expose_email,
        })
    }

    /// Serialize the webid response to a graph serializable syntax.
    pub fn try_into_turtle(resp: WebIdResponse) -> Result<String, ErrorResponse> {
        let webid: WebId = resp.webid.into();
        let profile_uri = WebId::resolve_webid_uri(&webid.user_id);
        let card_uri = WebId::resolve_webid_card_uri(
            &webid.user_id,
            RauthyConfig::get().pub_url_with_scheme.as_str(),
        );
        let t_user = NamedNodeRef::new(&profile_uri)?;
        let t_card = NamedNodeRef::new(&card_uri)?;

        let mut formatter = TurtleSerializer::new().for_writer(Vec::<u8>::new());
        formatter.serialize_triple(TripleRef::new(
            t_card,
            rdf::TYPE,
            Self::node(FOAF_PERSONAL_PROFILE_DOCUMENT),
        ))?;
        formatter.serialize_triple(TripleRef::new(
            t_card,
            Self::node(FOAF_PRIMARY_TOPIC),
            t_user,
        ))?;

        formatter.serialize_triple(TripleRef::new(
            t_user,
            Self::node(SOLID_OIDC_ISSUER),
            NamedNodeRef::new(&resp.issuer)?,
        ))?;
        formatter.serialize_triple(TripleRef::new(t_user, rdf::TYPE, Self::node(FOAF_PERSON)))?;

        // foaf:name
        if let Some(given_name) = &resp.given_name {
            formatter.serialize_triple(TripleRef::new(
                t_user,
                Self::node(FOAF_GIVEN_NAME),
                LiteralRef::new_simple_literal(given_name),
            ))?;
        }
        if let Some(family_name) = &resp.family_name {
            formatter.serialize_triple(TripleRef::new(
                t_user,
                Self::node(FOAF_FAMILY_NAME),
                LiteralRef::new_simple_literal(family_name),
            ))?;
        }

        // foaf:mbox
        if webid.expose_email {
            let mbox = format!("mailto:{}", resp.email);
            formatter.serialize_triple(TripleRef::new(
                t_user,
                Self::node(FOAF_MBOX),
                NamedNodeRef::new(&mbox)?,
            ))?;
        }

        // Custom triples are stored as N-Triples and appended to the open Turtle document.
        if let Some(custom_triples) = webid.custom_triples.as_deref() {
            for triple in NTriplesParser::new().for_slice(custom_triples.as_bytes()) {
                formatter.serialize_triple(triple?.as_ref())?;
            }
        }

        let finished = formatter.finish()?;
        Ok(String::from_utf8(finished).expect("Must be a valid turtle string."))
    }

    /// An IRI from this module's vocabulary constants; `oxrdf` validates every `NamedNode`,
    /// and these are static strings that cannot be invalid.
    fn node(iri: &str) -> NamedNodeRef<'_> {
        NamedNodeRef::new(iri).expect("Static WebID vocabulary IRI")
    }
}

impl From<WebId> for rauthy_api_types::users::WebId {
    fn from(value: WebId) -> Self {
        Self {
            user_id: value.user_id,
            expose_email: value.expose_email,
            custom_triples: value.custom_triples,
        }
    }
}

impl From<rauthy_api_types::users::WebId> for WebId {
    fn from(value: rauthy_api_types::users::WebId) -> Self {
        Self {
            user_id: value.user_id,
            expose_email: value.expose_email,
            custom_triples: value.custom_triples,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::webids::WebId;
    use rauthy_api_types::generic::Language;
    use rauthy_api_types::users::WebIdResponse;
    use rstest::rstest;

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
        let user_id = "SomeId123".to_owned();
        let expose_email = false;

        assert!(
            WebId::try_new(user_id, custom_data_turtle, expose_email, "localhost:8081").is_ok()
        );
    }

    #[rstest]
    #[case(
        Some(
            r#"
<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me>
<http://www.w3.org/ns/solid/terms#oidcIssuer>
<http://localhost:8080/auth/v1> .
"#
        ),
        // `oxttl` writes `rdf:type` with the Turtle keyword `a`.
        "<http://localhost:8081/auth/SomeId123/profile> a <http://xmlns.com/foaf/0.1/PersonalProfileDocument> ;\n\t<http://xmlns.com/foaf/0.1/primaryTopic> <http://localhost:8081/auth/SomeId123/profile#me> .\n<http://localhost:8081/auth/SomeId123/profile#me> <http://www.w3.org/ns/solid/terms#oidcIssuer> <http://localhost:8080/auth/v1> ;\n\ta <http://xmlns.com/foaf/0.1/Person> ;\n\t<http://xmlns.com/foaf/0.1/givenname> \"Given\" ;\n\t<http://xmlns.com/foaf/0.1/family_name> \"Family\" ;\n\t<http://xmlns.com/foaf/0.1/mbox> <mailto:mail@example.com> .\n<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me> <http://www.w3.org/ns/solid/terms#oidcIssuer> <http://localhost:8080/auth/v1> .\n"
    )]
    #[ignore] // this is currently ignored, because setting the PUB_URL here interferes with other tests in CI
    fn test_web_id_response(#[case] custom_triples: Option<&str>, #[case] expected_resp: &str) {
        let resp = WebIdResponse {
            webid: WebId::try_new(
                "SomeId123".to_string(),
                custom_triples,
                true,
                "localhost:8081",
            )
            .expect("Invalid custom triples in test case")
            .into(),
            issuer: "http://localhost:8080/auth/v1".to_string(),
            email: "mail@example.com".to_string(),
            given_name: Some("Given".to_string()),
            family_name: Some("Family".to_string()),
            language: Language::En,
        };

        let ttl = WebId::try_into_turtle(resp).unwrap();
        assert_eq!(ttl, expected_resp);
        // TODO we actually need real test cases with complex custom_triples to make sure
        // the outcome is as expected
    }
}
