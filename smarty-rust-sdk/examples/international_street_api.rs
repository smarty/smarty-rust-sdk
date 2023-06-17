extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::international_street_api::client::InternationalStreetClient;
use smarty_rust_sdk::international_street_api::lookup::Lookup;
use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        geocode: false,
        organization: "John Doe".to_string(),
        address1: "Rua Padre Antonio D'Angelo 121".to_string(),
        address2: "Casa Verde".to_string(),
        locality: "Sao Paulo".to_string(),
        administrative_area: "SP".to_string(),
        country: "Brazil".to_string(),
        postal_code: "02516-050".to_string(),
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID")?,
        std::env::var("SMARTY_AUTH_TOKEN")?,
    );

    let options = OptionsBuilder::new()
        .with_license("international-global-plus-cloud")
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = InternationalStreetClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}
