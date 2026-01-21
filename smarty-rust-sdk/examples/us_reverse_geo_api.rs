extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_reverse_geo_api::client::USReverseGeoClient;
use smarty_rust_sdk::us_reverse_geo_api::lookup::Lookup;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        latitude: 43.674389,
        longitude: -116.686195,
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

    let client = USReverseGeoClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);
    println!("Results Received: {}", lookup.results.results.len());

    Ok(())
}
