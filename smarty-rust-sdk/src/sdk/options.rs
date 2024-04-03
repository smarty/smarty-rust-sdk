use crate::sdk::authentication::Authenticate;

/// A builder for the options
///
/// Example:
/// ```ignore
/// let authentication = SecretKeyCredential::new("test".to_string(), "test".to_string());
///
/// OptionsBuilder::new(authentication)
///     .with_license("test_license")
///     .with_logging()
///     .build()
/// ```
pub struct OptionsBuilder {
    license: String,
    num_retries: u64,
    logging_enabled: bool,
    headers: Vec<(String, String)>,
    authentication: Box<dyn Authenticate>,
}

// Allowing this because it is a builder pattern
#[allow(clippy::new_without_default)]
impl OptionsBuilder {
    /// Creates a new OptionsBuilder, taking in the authentication for the options.
    pub fn new(authentication: Box<dyn Authenticate>) -> Self {
        Self {
            license: "".to_string(),
            num_retries: 10,
            logging_enabled: false,
            headers: vec![],
            authentication,
        }
    }

    /// Builds the builder into options with the parameters you set
    /// Returns an error if authentication is not set
    pub fn build(self) -> Options {
        Options {
            license: self.license,
            num_retries: self.num_retries,
            logging_enabled: self.logging_enabled,
            headers: self.headers,
            authentication: self.authentication,
        }
    }

    /// Adds a license string to the options
    pub fn with_license(mut self, license: &str) -> Self {
        self.license = license.to_string();
        self
    }

    /// Forces a maximum number of retries that a request will attempt to handle.
    pub fn with_retries(mut self, num_retries: u64) -> Self {
        self.num_retries = num_retries;
        self
    }

    /// Enables Logging
    pub fn with_logging(mut self) -> Self {
        self.logging_enabled = true;
        self
    }

    /// Adds a set of custom headers to your request.
    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers;
        self
    }
}

/// Options that can be passed into a new client
/// <num_retries>: the number of retries that the client with run before giving up.
/// <logging_enabled>: whether we should send logging data
/// <headers>: Custom headers that you can pass in
/// <authentication>: A authentication for Smarty
pub struct Options {
    pub(crate) license: String,

    // Retry Sender
    pub(crate) num_retries: u64,

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
