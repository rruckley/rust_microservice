//! Payload Module
//! 
use serde::{Deserialize,Serialize};
use crate::error::Error;

/// Defined input payload(s) here. Can reference external libraries such as TMFLib
#[derive(Clone,Debug,Default,Deserialize)]
pub struct RequestPayload {}

impl RequestPayload {
    /// Sample function to process the request payload and produce as a result
    /// a response payload or an error.
    pub fn process(self) -> Result<ResponsePayload,Error> {
        // Convert Input to put via some logic
        Ok(ResponsePayload {})
    }
}

/// Defined output payloads here. Can reference external libraries such as TMFLib
#[derive(Clone,Debug,Default,Serialize)]
pub struct ResponsePayload {}