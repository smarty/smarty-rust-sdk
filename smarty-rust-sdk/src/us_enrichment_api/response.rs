use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait EnrichmentResponse: Clone + Serialize + DeserializeOwned + Default {
    fn lookup_type() -> &'static str;
}
