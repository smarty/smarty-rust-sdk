extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::international_street_api::client::InternationalStreetClient;
use smarty_rust_sdk::international_street_api::lookup::Lookup;
use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::Options;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup = &mut Lookup {
        geocode:            false,
        organization:       "John Doe".to_string(),
        address1:           "Rua Padre Antonio D'Angelo 121".to_string(),
        address2:           "Casa Verde".to_string(),
        locality:           "Sao Paulo".to_string(),
        administrative_area: "SP".to_string(),
        country:            "Brazil".to_string(),
        postal_code:         "02516-050".to_string(),
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(std::env::var("SMARTY_AUTH_ID")?, std::env::var("SMARTY_AUTH_TOKEN")?);

    let mut options = Options::default();
    options.license = "international-global-plus-cloud".to_string();

    options.authentication = authentication;

    let client = InternationalStreetClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}