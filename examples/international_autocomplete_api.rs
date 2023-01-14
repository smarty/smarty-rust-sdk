extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::international_autocomplete_api::client::InternationalAutocompleteClient;
use smarty_rust_sdk::international_autocomplete_api::lookup::Lookup;
use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::Options;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup = &mut Lookup {
        country:  "FRA".to_string(),
        search:   "Louis".to_string(),
        include_only_locality: "Paris".to_string(),
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(std::env::var("SMARTY_AUTH_ID")?, std::env::var("SMARTY_AUTH_TOKEN")?);

    let mut options = Options::new();

    options.authentication = authentication;

    let client = InternationalAutocompleteClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}