pub mod client;
pub mod lookup;
pub mod results;

#[cfg(test)]
mod tests {
    use crate::{
        sdk::{authentication::SecretKeyCredential, options::OptionsBuilder},
        us_enrichment_api::client::USEnrichmentClient,
    };

    #[test]
    fn client_test() {
        let options =
            OptionsBuilder::new(SecretKeyCredential::new("".to_string(), "".to_string())).build();
        let client = USEnrichmentClient::new(options).unwrap();

        assert_eq!(
            client.client.url.to_string(),
            "https://us-enrichment.api.smarty.com/lookup".to_string()
        )
    }
}
