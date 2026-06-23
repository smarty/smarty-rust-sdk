extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

// This example is for US Autocomplete (V2). It has the same name as a previous product
// which has been deprecated since 2022, which we refer to as US Autocomplete Basic.
// If you are still using US Autocomplete Basic, this SDK will not work.

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_autocomplete_api::client::USAutocompleteClient;
use smarty_rust_sdk::us_autocomplete_api::lookup::{Lookup, PreferGeolocation, Source};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Documentation for input fields can be found at:
    // https://www.smarty.com/docs/apis/us-autocomplete-v2/reference#http-request-input-fields

    let lookup = &mut Lookup {
        search: "1042 W Center".to_string(),
        max_results: 5,
        city_filter: vec!["Orem,UT".to_string(), "Madisonville,KY".to_string()],
        state_filter: vec!["UT".to_string(), "KY".to_string()],
        prefer_state: vec!["KY".to_string()],
        prefer_ratio: 3,
        prefer_geolocation: PreferGeolocation::GeolocateCity,
        source: Some(Source::All),
        ..Default::default()
    };

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = USAutocompleteClient::new(options)?;
    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    // Expand the secondaries of a result that has an entry_id by passing it back as the selected address.
    if let Some(entry_id) = lookup
        .results
        .suggestions
        .iter()
        .find(|suggestion| !suggestion.entry_id.is_empty())
        .map(|suggestion| suggestion.entry_id.clone())
    {
        lookup.selected = entry_id;
        client.send(lookup).await?;

        println!("Secondaries:");
        println!("{}", serde_json::to_string_pretty(&lookup.results)?);
    }

    Ok(())
}
