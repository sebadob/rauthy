use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum LogLevelAccess {
    Debug,
    Verbose,
    Basic,
    Modifying,
    Off,
}

impl FromStr for LogLevelAccess {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Self::Debug),
            "verbose" => Ok(Self::Verbose),
            "basic" => Ok(Self::Basic),
            "modifying" => Ok(Self::Modifying),
            "off" => Ok(Self::Off),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Cannot parse LogLevelAccess",
            )),
        }
    }
}
