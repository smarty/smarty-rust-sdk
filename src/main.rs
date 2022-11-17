use std::error::Error;
use reqwest::{Method};
use crate::builder::ClientBuilder;
use tokio;
use std::env;
use std::env::VarError;

mod builder;
mod candidate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let query = "&street=1600+amphitheatre+pkwy&city=mountain+view&state=CA&candidates=10";

    let url = format!("https://us-street.api.smartystreets.me/street-address?auth-id={auth_id}&auth-token={auth_token}&license={license}{query}",
                      auth_id = auth.auth_token,
                      auth_token = auth.auth_id,
                      query = query,
                      license = "us-core-cloud");

    println!("{}", url);

    let mut client_builder = ClientBuilder::new(url.as_str())?;
    let client = client_builder.build_httpclient();

    let req = client.request(Method::GET, url.as_str());
    let content = req.send()
        .await?
        .text()
        .await?;

    println!("text: {:?}", content);

    Ok(())
}

pub struct Authentication {
    auth_id: String,
    auth_token: String,
}

impl Authentication {
    pub fn new(auth_id_env: &str, auth_token_env: &str) -> Result<Authentication, VarError> {
        let auth_token = env::var(auth_id_env)?;
        let auth_id = env::var(auth_token_env)?;

        let authentication = Authentication { auth_id, auth_token };

        Ok(authentication)
    }
}
