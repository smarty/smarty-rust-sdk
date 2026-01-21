extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_autocomplete_pro_api::client::USAutocompleteProClient;
use smarty_rust_sdk::us_autocomplete_pro_api::lookup::{Geolocation, Lookup};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        search: "1042 W Center".to_string(),
        max_results: 5,
        city_filter: vec!["Orem,UT".to_string(), "Madisonville,KY".to_string()],
        state_filter: vec!["UT".to_string(), "KY".to_string()],
        prefer_state: vec!["KY".to_string()],
        prefer_ratio: 3,
        geolocation: Geolocation::GeolocateCity,
        source: "all".to_string(),
        ..Default::default()
    };

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = USAutocompleteProClient::new(options)?;
    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}
