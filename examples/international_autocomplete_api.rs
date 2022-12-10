extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::international_autocomplete_api::client::InternationalAutocompleteClient;
use smarty_rust_sdk::international_autocomplete_api::lookup::Lookup;
use smarty_rust_sdk::sdk::authentication::Authentication;
use smarty_rust_sdk::sdk::options::Options;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup = &mut Lookup {
        country:  "FRA".to_string(),
        search:   "Louis".to_string(),
        locality: "Paris".to_string(),
        ..Default::default()
    };

    let authentication = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let mut options = Options::new();
    options.auth_id = authentication.auth_id.to_string();
    options.auth_token = authentication.auth_token.to_string();

    let client = InternationalAutocompleteClient::new("https://international-autocomplete.api.smartystreets.me".parse()?, options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}