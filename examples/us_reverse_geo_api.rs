extern crate smarty_rust_sdk;
extern crate tokio;

use std::error::Error;
use smarty_rust_sdk::sdk::authentication::Authentication;
use smarty_rust_sdk::sdk::options::Options;
use smarty_rust_sdk::us_reverse_geo::client::USReverseGeoClient;
use smarty_rust_sdk::us_reverse_geo::lookup::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let lookup = &mut Lookup {
        latitude: 40.241336927519136,
        longitude: -111.70786647260555,
        ..Default::default()
    };

    let authentication = Authentication::new("SMARTY_AUTH_ID", "SMARTY_AUTH_TOKEN")?;

    let mut options = Options::new();
    options.auth_id = authentication.auth_id.to_string();
    options.auth_token = authentication.auth_token.to_string();
    options.license = "us-reverse-geocoding-cloud".to_string();

    let client = USReverseGeoClient::new("https://us-reverse-geo.api.smartystreets.me/".parse()?, options)?;

    client.send(lookup).await?;

    println!("{:?}", lookup.results);

    Ok(())
}