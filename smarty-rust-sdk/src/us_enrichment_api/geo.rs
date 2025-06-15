use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

use super::response::EnrichmentResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoReferenceResponse {
    pub smarty_key: String,
    pub data_set_name: String,
    pub data_set_version: String,
    pub attributes: GeoReferenceAttributes,
}

impl EnrichmentResponse for GeoReferenceResponse {
    fn lookup_type() -> &'static str {
        "geo-reference"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoReference2010Response(pub GeoReferenceResponse);

impl Deref for GeoReference2010Response {
    type Target = GeoReferenceResponse;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GeoReference2010Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl EnrichmentResponse for GeoReference2010Response {
    fn lookup_type() -> &'static str {
        "geo-reference/2010"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoReference2020Response(pub GeoReferenceResponse);

impl Deref for GeoReference2020Response {
    type Target = GeoReferenceResponse;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GeoReference2020Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl EnrichmentResponse for GeoReference2020Response {
    fn lookup_type() -> &'static str {
        "geo-reference/2020"
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoReferenceAttributes {
    pub census_block: CensusBlock,
    pub census_county_division: CensusCountyDivision,
    pub census_tract: CensusTract,
    pub core_based_stat_area: CoreBasedStatArea,
    pub place: Place,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CensusBlock {
    pub accuracy: String,
    pub geoid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CensusCountyDivision {
    pub accuracy: String,
    pub code: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CensusTract {
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CoreBasedStatArea {
    pub code: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Place {
    pub accuracy: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: String,
}
