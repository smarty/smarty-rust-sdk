extern crate smarty_rust_sdk;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
use smarty_rust_sdk::sdk::authentication::Authentication;
use smarty_rust_sdk::sdk::Geolocation;
use smarty_rust_sdk::sdk::options::Options;
use smarty_rust_sdk::us_autocomplete_pro::client::USAutocompleteProClient;
use smarty_rust_sdk::us_autocomplete_pro::lookup::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup = &mut Lookup {
        search: "1042 W Center".to_string(),
        max_results: 5,
        city_filter: vec!("Denver".to_string(), "Orem".to_string()),
        state_filter: vec!("CO".to_string(), "UT".to_string()),
        prefer_state: vec!("CO".to_string()),
        prefer_ratio: 3,
        geolocation: Geolocation::GeolocateCity,
        source: "all".to_string(),
        ..Default::default()
    };

    let authentication = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let mut options = Options::new();
    options.auth_id = authentication.auth_id.to_string();
    options.auth_token = authentication.auth_token.to_string();
    options.license = "us-autocomplete-pro-cloud".to_string();

    let client = USAutocompleteProClient::new("https://us-autocomplete-pro.api.smartystreets.me/".parse()?, options)?;
    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}