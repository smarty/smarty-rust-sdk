extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::Options;
use smarty_rust_sdk::us_reverse_geo_api::client::USReverseGeoClient;
use smarty_rust_sdk::us_reverse_geo_api::lookup::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        latitude: 40.27644,
        longitude: -111.65747,
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(std::env::var("SMARTY_AUTH_ID")?, std::env::var("SMARTY_AUTH_TOKEN")?);

    let mut options = Options::default();
    options.license = "us-reverse-geocoding-cloud".to_string();

    options.authentication = authentication;

    let client = USReverseGeoClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);
    println!("Results Received: {}", lookup.results.results.len());

    Ok(())
}