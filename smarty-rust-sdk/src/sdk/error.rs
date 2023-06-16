use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]

/// A Simple Error that returns an error code, if there is one, and the details that were recieved.
pub struct SDKError {
    pub code: Option<u16>,
    pub detail: Option<String>
}

impl Display for SDKError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SDK Error: ErrorCode: {:?}\nDetails: {:?}", self.code, self.detail)
    }
}

impl Error for SDKError {}