use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use hiqlite_macros::params;
use rauthy_api_types::users::WebIdResponse;
use rauthy_common::constants::CACHE_TTL_USER;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use rio_api::formatter::TriplesFormatter;
use rio_api::model::{Literal, NamedNode, Subject, Term, Triple};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesFormatter, NTriplesParser, TurtleFormatter, TurtleParser};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub expose_email: bool,
    pub custom_triples: Option<String>,
}

impl From<tokio_postgres::Row> for WebId {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            expose_email: row.get("expose_email"),
            custom_triples: row.get("custom_triples"),
        }
    }
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
            let mut ttl_parser = TurtleParser::new(
                data.as_bytes(),
                Some(Self::resolve_webid_card_uri(&user_id, pub_url).parse()?),
            );
            let mut ntriples_fmt = NTriplesFormatter::new(Vec::<u8>::new());
            ttl_parser.parse_all(&mut |t| ntriples_fmt.format(&t))?;

            // as long as the formatter itself is not buggy, this String conversion should never panic
            Some(String::from_utf8(ntriples_fmt.finish()?).expect("Must be valid string."))
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

    fn triple<'a>(s: impl Into<Subject<'a>>, p: &'a str, o: impl Into<Term<'a>>) -> Triple<'a> {
        Triple {
            subject: s.into(),
            predicate: NamedNode { iri: p },
            object: o.into(),
        }
    }

    /// Serialize the webid response to a graph serializable syntax.
    fn serialize_turtle(
        resp: WebIdResponse,
        formatter: &mut TurtleFormatter<Vec<u8>>,
    ) -> Result<(), ErrorResponse> {
        let webid: WebId = resp.webid.into();
        let t_user = NamedNode {
            iri: &WebId::resolve_webid_uri(&webid.user_id),
        };
        let t_card = NamedNode {
            iri: &WebId::resolve_webid_card_uri(
                &webid.user_id,
                RauthyConfig::get().pub_url_with_scheme.as_str(),
            ),
        };
        let t_type = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

        formatter.format(&Self::triple(
            t_card,
            t_type,
            NamedNode {
                iri: "http://xmlns.com/foaf/0.1/PersonalProfileDocument",
            },
        ))?;

        formatter.format(&Self::triple(
            t_card,
            "http://xmlns.com/foaf/0.1/primaryTopic",
            t_user,
        ))?;

        formatter.format(&Self::triple(
            t_user,
            "http://www.w3.org/ns/solid/terms#oidcIssuer",
            NamedNode { iri: &resp.issuer },
        ))?;

        // rdf:type
        formatter.format(&Self::triple(
            t_user,
            t_type,
            NamedNode {
                iri: "http://xmlns.com/foaf/0.1/Person",
            },
        ))?;

        // foaf:name
        formatter.format(&Self::triple(
            t_user,
            "http://xmlns.com/foaf/0.1/givenname",
            Literal::Simple {
                value: &resp.given_name,
            },
        ))?;
        if let Some(value) = &resp.family_name {
            formatter.format(&Self::triple(
                t_user,
                "http://xmlns.com/foaf/0.1/family_name",
                Literal::Simple { value },
            ))?;
        }

        // foaf:mbox
        if webid.expose_email {
            formatter.format(&Self::triple(
                t_user,
                "http://xmlns.com/foaf/0.1/mbox",
                NamedNode {
                    iri: &format!("mailto:{}", &resp.email),
                },
            ))?;
        }

        // Format custom data.
        webid
            .fmt_custom_triples_to_ttl(formatter)
            .expect("Must be valid");

        Ok(())
    }

    pub fn try_into_turtle(resp: WebIdResponse) -> Result<String, ErrorResponse> {
        let mut formatter = TurtleFormatter::new(Vec::<u8>::new());
        WebId::serialize_turtle(resp, &mut formatter)?;

        let finished = formatter.finish()?;
        Ok(String::from_utf8(finished).expect("Must be a valid turtle string."))
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
        "<http://localhost:8081/auth/SomeId123/profile> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/PersonalProfileDocument> ;\n\t<http://xmlns.com/foaf/0.1/primaryTopic> <http://localhost:8081/auth/SomeId123/profile#me> .\n<http://localhost:8081/auth/SomeId123/profile#me> <http://www.w3.org/ns/solid/terms#oidcIssuer> <http://localhost:8080/auth/v1> ;\n\t<http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> ;\n\t<http://xmlns.com/foaf/0.1/givenname> \"Given\" ;\n\t<http://xmlns.com/foaf/0.1/family_name> \"Family\" ;\n\t<http://xmlns.com/foaf/0.1/mbox> <mailto:mail@example.com> .\n<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me> <http://www.w3.org/ns/solid/terms#oidcIssuer> <http://localhost:8080/auth/v1> .\n"
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
            given_name: "Given".to_string(),
            family_name: Some("Family".to_string()),
            language: Language::En,
        };

        let ttl = WebId::try_into_turtle(resp).unwrap();
        assert_eq!(ttl, expected_resp);
        // TODO we actually need real test cases with complex custom_triples to make sure
        // the outcome is as expected
    }
}
