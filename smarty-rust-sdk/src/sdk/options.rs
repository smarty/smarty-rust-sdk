use crate::sdk::authentication::Authenticate;

use super::error::SDKError;

/// Builds out the options for your client
pub struct OptionsBuilder {
    license: String,
    num_retries: u32,
    logging_enabled: bool,
    headers: Vec<(String, String)>,
    authentication: Option<Box<dyn Authenticate>>,
}

// Allowing this because it is a builder pattern
#[allow(clippy::new_without_default)]
impl OptionsBuilder {
    pub fn new() -> Self {
        Self {
            license: "".to_string(),
            num_retries: u32::default(),
            logging_enabled: false,
            headers: vec![],
            authentication: None,
        }
    }

    /// Builds the builder into options with the parameters you set
    /// Returns an error if authentication is not set
    pub fn build(self) -> Result<Options, SDKError> {
        if let Some(auth) = self.authentication {
            return Ok(Options {
                license: self.license,
                num_retries: self.num_retries,
                logging_enabled: self.logging_enabled,
                headers: self.headers,
                authentication: auth,
            });
        }
        Err(SDKError {
            code: None,
            detail: Some("Authentication Required".to_string()),
        })
    }
    pub fn with_license(mut self, license: &str) -> Self {
        self.license = license.to_string();
        self
    }
    pub fn with_retries(mut self, num_retries: u32) -> Self {
        self.num_retries = num_retries;
        self
    }
    /// Enables Logging on these options
    pub fn with_logging(mut self) -> Self {
        self.logging_enabled = true;
        self
    }
    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers;
        self
    }
    pub fn authenticate(mut self, authentication: Box<dyn Authenticate>) -> Self {
        self.authentication = Some(authentication);
        self
    }
}

/// Options that can be passed into a new client
/// num_retries: the number of retries that the client with run before giving up.
/// logging_enabled: whether we should send logging data
/// headers: Custom headers that you can pass in
/// authentication: A authentication for Smarty
pub struct Options {
    pub(crate) license: String,

    // Retry Sender
    pub(crate) num_retries: u32,

    // Logger
    pub(crate) logging_enabled: bool,

    // Custom Headers
    pub(crate) headers: Vec<(String, String)>,

    // Authentication
    pub(crate) authentication: Box<dyn Authenticate>,
}

impl Clone for Options {
    fn clone(&self) -> Self {
        Self {
            license: self.license.clone(),
            num_retries: self.num_retries,
            logging_enabled: self.logging_enabled,
            headers: self.headers.clone(),
            authentication: self.authentication.clone_box(),
        }
    }
}
