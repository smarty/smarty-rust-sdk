use std::env;
use std::env::VarError;

pub struct Authentication {
    pub auth_id: String,
    pub auth_token: String,
}

impl Authentication {
    pub fn new(auth_id_env: &str, auth_token_env: &str) -> Result<Authentication, VarError> {
        let auth_token = env::var(auth_token_env)?;
        let auth_id = env::var(auth_id_env)?;

        let authentication = Authentication { auth_id, auth_token };

        Ok(authentication)
    }
}