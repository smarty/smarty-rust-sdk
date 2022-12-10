use reqwest_middleware::RequestBuilder;

pub trait Authenticate {
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder;
}

pub struct SecretKeyCredential {
    pub auth_id: String,
    pub auth_token: String,
}

impl SecretKeyCredential {
    pub fn new(auth_id: String, auth_token: String) -> Box<SecretKeyCredential> {
        Box::new(SecretKeyCredential { auth_id, auth_token })
    }
}

impl Authenticate for SecretKeyCredential {
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder {
        request.query(&[("auth-id".to_string(), self.auth_id.clone()), ("auth-token".to_string(), self.auth_token.clone())])
    }
}

pub struct WebsiteKeyCredential {
    key: String,
    host: String
}

impl WebsiteKeyCredential {
    pub fn new(key: &str, host: &str) -> Self {
        Self {
            key: key.to_string(),
            host: host.to_string()
        }
    }
}

impl Authenticate for WebsiteKeyCredential {
    fn authenticate(&self, mut request: RequestBuilder) -> RequestBuilder {
        request = request.query(&[("key".to_string(), self.key.clone())]);
        request = request.header("Referer".to_string(), self.host.clone());
        request
    }
}