extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_autocomplete_pro_api::client::USAutocompleteProClient;
use smarty_rust_sdk::us_autocomplete_pro_api::lookup::{Geolocation, Lookup};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        search: "1042 W Center".to_string(),
        max_results: 5,
        city_filter: vec!["Denver".to_string(), "Orem".to_string()],
        state_filter: vec!["CO".to_string(), "UT".to_string()],
        prefer_state: vec!["CO".to_string()],
        prefer_ratio: 3,
        geolocation: Geolocation::GeolocateCity,
        source: "all".to_string(),
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID")?,
        std::env::var("SMARTY_AUTH_TOKEN")?,
    );

    let options = OptionsBuilder::new()
        .with_license("us-autocomplete-pro-cloud")
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = USAutocompleteProClient::new(options)?;
    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}
