//! Error Module
use serde::Serialize;

#[derive(Clone,Debug,Default,Serialize)]
pub struct Error {
    code : String,
    message : String,
}

const ERROR_CODE_GENERIC : u16 = 1001;

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error {
            code : ERROR_CODE_GENERIC.to_string(),
            message : value,
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::from(value.to_string())
    }
}