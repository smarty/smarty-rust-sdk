use crate::sdk::has_param;

#[derive(Clone)]
pub struct Options {
    pub auth_id: String,
    pub auth_token: String,
    pub license: String,

    // Retry Sender
    pub num_retries: u32,

    // Logger
    pub logging_enabled: bool
}

impl Options {
    pub fn new() -> Self {
        Self {
            auth_id: String::default(),
            auth_token: String::default(),
            license: String::default(),
            num_retries: u32::default(),
            logging_enabled: false
        }
    }

    pub fn to_param_array(self) -> String {
        let test = vec![
            Some(("auth-id".to_string(), self.auth_id)),
            Some(("auth-token".to_string(), self.auth_token)),
            has_param("license".to_string(), self.license),
        ]
            .iter()
            .filter_map(Option::clone)
            .map(|x| x.0.clone() + "=" + x.1.clone().as_str())
            .collect::<Vec<String>>();

        test.join("&")
    }
}
