extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_reverse_geo_api::client::USReverseGeoClient;
use smarty_rust_sdk::us_reverse_geo_api::lookup::Lookup;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        latitude: 40.27644,
        longitude: -111.65747,
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID")?,
        std::env::var("SMARTY_AUTH_TOKEN")?,
    );

    let options = OptionsBuilder::new()
        .with_license("us-reverse-geocoding-cloud")
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = USReverseGeoClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);
    println!("Results Received: {}", lookup.results.results.len());

    Ok(())
}
