extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::international_autocomplete_api::client::InternationalAutocompleteClient;
use smarty_rust_sdk::international_autocomplete_api::lookup::Lookup;
use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        country: "FRA".to_string(),
        search: "Louis".to_string(),
        include_only_locality: "Paris".to_string(),
        ..Default::default()
    };

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = InternationalAutocompleteClient::new(options)?;

    client.send(lookup).await?;

    println!(
        "Original Value: {}",
        serde_json::to_string_pretty(&lookup.results)?
    );

    for result in &lookup.results.suggestions {
        let additional_lookup = &mut Lookup {
            country: "FRA".to_string(),
            address_id: result.address_id.clone(),
            max_results: result.entries,
            ..Default::default()
        };

        client.send(additional_lookup).await?;

        println!(
            "Entries for {} {}",
            serde_json::to_string_pretty(&result)?,
            serde_json::to_string_pretty(&additional_lookup.results)?
        );
    }

    Ok(())
}
