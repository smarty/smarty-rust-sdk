use reqwest_middleware::RequestBuilder;
use std::fmt::Debug;

/// Allows for cloning of an authentication system
pub trait AuthClone {
    fn clone_box(&self) -> Box<dyn Authenticate>;
}

/// What the authentication does to the request in order to
/// authenticate the client.
pub trait Authenticate: Sync + Send + Debug + AuthClone {
    /// Authenticates the Request with the given authentication credentials
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder;
}

#[derive(Clone, PartialEq, Debug)]
pub struct SecretKeyCredential {
    pub auth_id: String,
    pub auth_token: String,
}

impl SecretKeyCredential {
    pub fn new(auth_id: String, auth_token: String) -> Box<Self> {
        Box::new(SecretKeyCredential {
            auth_id,
            auth_token,
        })
    }
}

impl AuthClone for SecretKeyCredential {
    fn clone_box(&self) -> Box<dyn Authenticate> {
        Box::new(self.clone())
    }
}

impl Authenticate for SecretKeyCredential {
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder {
        request.query(&[
            ("auth-id".to_string(), self.auth_id.clone()),
            ("auth-token".to_string(), self.auth_token.clone()),
        ])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WebsiteKeyCredential {
    key: String,
    host: String,
}

impl WebsiteKeyCredential {
    pub fn new(key: &str, host: &str) -> Box<Self> {
        Box::new(Self {
            key: key.to_string(),
            host: host.to_string(),
        })
    }
}

impl AuthClone for WebsiteKeyCredential {
    fn clone_box(&self) -> Box<dyn Authenticate> {
        Box::new(self.clone())
    }
}

impl Authenticate for WebsiteKeyCredential {
    fn authenticate(&self, mut request: RequestBuilder) -> RequestBuilder {
        request = request.query(&[("key".to_string(), self.key.clone())]);
        request = request.header(reqwest::header::REFERER, self.host.clone());
        request
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BasicAuthCredential {
    auth_id: String,
    auth_token: String,
}

impl BasicAuthCredential {
    pub fn new(auth_id: String, auth_token: String) -> Box<Self> {
        if auth_id.is_empty() || auth_token.is_empty() {
            panic!("credentials (auth id, auth token) required");
        }
        Box::new(BasicAuthCredential {
            auth_id,
            auth_token,
        })
    }
}

impl AuthClone for BasicAuthCredential {
    fn clone_box(&self) -> Box<dyn Authenticate> {
        Box::new(self.clone())
    }
}

impl Authenticate for BasicAuthCredential {
    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(&self.auth_id, Some(&self.auth_token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_basic_auth_credential_with_valid_credentials() {
        let cred = BasicAuthCredential::new("testID".to_string(), "testToken".to_string());

        assert_eq!(cred.auth_id, "testID");
        assert_eq!(cred.auth_token, "testToken");
    }

    #[test]
    #[should_panic(expected = "credentials (auth id, auth token) required")]
    fn test_new_basic_auth_credential_with_empty_auth_id() {
        BasicAuthCredential::new("".to_string(), "testToken".to_string());
    }

    #[test]
    #[should_panic(expected = "credentials (auth id, auth token) required")]
    fn test_new_basic_auth_credential_with_empty_auth_token() {
        BasicAuthCredential::new("testID".to_string(), "".to_string());
    }

    #[test]
    #[should_panic(expected = "credentials (auth id, auth token) required")]
    fn test_new_basic_auth_credential_with_both_empty() {
        BasicAuthCredential::new("".to_string(), "".to_string());
    }

    #[test]
    fn test_new_basic_auth_credential_with_special_characters() {
        let cred = BasicAuthCredential::new("test@id#123".to_string(), "token!@#$%^&*()".to_string());

        assert_eq!(cred.auth_id, "test@id#123");
        assert_eq!(cred.auth_token, "token!@#$%^&*()");
    }

    #[test]
    fn test_new_basic_auth_credential_with_password_containing_colon() {
        // Note: Per RFC 2617, userid must NOT contain colons, but password can
        let cred = BasicAuthCredential::new("validUserID".to_string(), "password:with:colons".to_string());

        assert_eq!(cred.auth_id, "validUserID");
        assert_eq!(cred.auth_token, "password:with:colons");
    }

    #[test]
    fn test_new_basic_auth_credential_with_unicode_characters() {
        let cred = BasicAuthCredential::new("用户".to_string(), "密码".to_string());

        assert_eq!(cred.auth_id, "用户");
        assert_eq!(cred.auth_token, "密码");
    }

    #[test]
    fn test_basic_auth_credential_authenticate() {
        use reqwest_middleware::ClientBuilder;

        let cred = BasicAuthCredential::new("myID".to_string(), "myToken".to_string());
        let client = ClientBuilder::new(reqwest::Client::new()).build();
        let req = client.request(reqwest::Method::GET, "http://example.com");

        let authenticated_req = cred.authenticate(req);

        // Build the request to verify the Authorization header is set
        let built = authenticated_req.build().unwrap();
        let auth_header = built.headers().get(reqwest::header::AUTHORIZATION);

        assert!(auth_header.is_some());
        // Basic auth header format is "Basic base64(username:password)"
        let auth_value = auth_header.unwrap().to_str().unwrap();
        assert!(auth_value.starts_with("Basic "));
    }

    #[test]
    fn test_basic_auth_credential_authenticate_with_special_characters() {
        use reqwest_middleware::ClientBuilder;

        let cred = BasicAuthCredential::new("user@domain.com".to_string(), "p@ssw0rd!".to_string());
        let client = ClientBuilder::new(reqwest::Client::new()).build();
        let req = client.request(reqwest::Method::GET, "http://example.com");

        let authenticated_req = cred.authenticate(req);

        let built = authenticated_req.build().unwrap();
        let auth_header = built.headers().get(reqwest::header::AUTHORIZATION);

        assert!(auth_header.is_some());
        let auth_value = auth_header.unwrap().to_str().unwrap();
        assert!(auth_value.starts_with("Basic "));
    }

    #[test]
    fn test_basic_auth_credential_clone() {
        let cred = BasicAuthCredential::new("testID".to_string(), "testToken".to_string());
        let cloned = cred.clone();

        assert_eq!(cred.auth_id, cloned.auth_id);
        assert_eq!(cred.auth_token, cloned.auth_token);
    }

    #[test]
    fn test_basic_auth_credential_clone_box() {
        use reqwest_middleware::ClientBuilder;

        let cred = BasicAuthCredential::new("testID".to_string(), "testToken".to_string());
        let cloned_box = cred.clone_box();

        // Verify the cloned box is a valid Authenticate trait object
        let client = ClientBuilder::new(reqwest::Client::new()).build();
        let req = client.request(reqwest::Method::GET, "http://example.com");
        let authenticated_req = cloned_box.authenticate(req);
        let built = authenticated_req.build().unwrap();

        assert!(built.headers().get(reqwest::header::AUTHORIZATION).is_some());
    }
}
