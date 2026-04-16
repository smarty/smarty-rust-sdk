use serde::de::DeserializeOwned;
use serde::Serialize;

/// Describes how an enrichment endpoint's URL path is structured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointPathKind {
    /// /lookup/{smarty_key}/{type} or /lookup/search/{type}
    Standard,
    /// /lookup/{type}/{business_id}
    BusinessId,
}

pub trait EnrichmentResponse: Clone + Serialize + DeserializeOwned + Default {
    fn lookup_type() -> &'static str;

    fn path_kind() -> EndpointPathKind {
        EndpointPathKind::Standard
    }
}
