pub mod client;
pub mod lookup;

pub mod response;

pub mod geo;
pub mod principal;
pub mod secondary;
pub mod risk;

#[cfg(test)]
mod tests {
    use crate::{sdk::options::OptionsBuilder, us_enrichment_api::client::USEnrichmentClient};

    #[test]
    fn client_test() {
        let options = OptionsBuilder::new(None).build();
        let client = USEnrichmentClient::new(options).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-enrichment.api.smarty.com/lookup".to_string()
        )
    }
}
