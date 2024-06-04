use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationToken {
    pub id: String,
    pub ts: i64,
}

impl Display for ContinuationToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.ts, self.id)
    }
}

impl TryFrom<&str> for ContinuationToken {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 16 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid continuation_token",
            ));
        }

        let (ts, id) = value.split_at(10);
        let ts = i64::from_str(ts).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("timestamp str cannot be parsed into i64: {:?}", err),
            )
        })?;

        Ok(Self {
            id: id.to_string(),
            ts,
        })
    }
}

impl ContinuationToken {
    pub fn new(id: String, ts: i64) -> Self {
        Self { id, ts }
    }

    pub fn into_header_pair(self) -> (HeaderName, HeaderValue) {
        // these header values will always be valid
        let name = HeaderName::from_str("x-continuation-token").unwrap();
        let value = HeaderValue::from_str(&self.to_string()).unwrap();
        (name, value)
    }
}
