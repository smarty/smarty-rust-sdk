extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::international_postal_code_api::client::InternationalPostalCodeClient;
use smarty_rust_sdk::international_postal_code_api::lookup::Lookup;
use smarty_rust_sdk::sdk::authentication::BasicAuthCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        input_id: "ID-8675309".to_string(),
        locality: "Sao Paulo".to_string(),
        administrative_area: "SP".to_string(),
        country: "Brazil".to_string(),
        postal_code: "02516".to_string(),
        ..Default::default()
    };

    let authentication = BasicAuthCredential::new(
        std::env::var("SMARTY_AUTH_ID").expect("Missing SMARTY_AUTH_ID env variable"),
        std::env::var("SMARTY_AUTH_TOKEN").expect("Missing SMARTY_AUTH_TOKEN env variable"),
    );

    let options = OptionsBuilder::new(Some(authentication))
        .with_logging()
        .build();

    let client = InternationalPostalCodeClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.results)?);

    Ok(())
}
