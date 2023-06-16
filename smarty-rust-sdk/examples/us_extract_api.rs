extern crate serde_json;
extern crate smarty_rust_sdk;
extern crate tokio;

use smarty_rust_sdk::sdk::authentication::SecretKeyCredential;
use smarty_rust_sdk::sdk::options::OptionsBuilder;
use smarty_rust_sdk::us_extract_api::client::USExtractClient;
use smarty_rust_sdk::us_extract_api::lookup::Lookup;
use smarty_rust_sdk::us_street_api::lookup::MatchStrategy;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lookup = &mut Lookup {
        text: "Meet me at 3214 N University Ave Provo UT 84604 just after 3pm.\n\
               Or! We could meet at 1600 Pennsylvania Ave, Washington DC."
            .to_string(),
        aggressive: true,
        addresses_with_line_breaks: true,
        addresses_per_line: 2,
        match_strategy: MatchStrategy::Enhanced,
        ..Default::default()
    };

    let authentication = SecretKeyCredential::new(
        std::env::var("SMARTY_AUTH_ID")?,
        std::env::var("SMARTY_AUTH_TOKEN")?,
    );

    let options = OptionsBuilder::new()
        .with_license("us-core-cloud")
        .with_logging()
        .authenticate(authentication)
        .build()
        .unwrap();

    let client = USExtractClient::new(options)?;

    client.send(lookup).await?;

    println!("{}", serde_json::to_string_pretty(&lookup.result)?);

    Ok(())
}
