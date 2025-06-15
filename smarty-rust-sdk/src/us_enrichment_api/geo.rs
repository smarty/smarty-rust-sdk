use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

use super::response::EnrichmentResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct GeoReferenceAttributes {
    pub census_block: CensusBlock,
    pub census_county_division: CensusCountyDivision,
    pub census_tract: CensusTract,
    pub core_based_stat_area: CoreBasedStatArea,
    pub place: Place,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CensusBlock {
    pub accuracy: CensusGeoIdAccuracy,
    pub geoid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CensusGeoIdAccuracy {
    #[default]
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "tract")]
    Tract,
    #[serde(rename = "county")]
    County,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeocodeAccuracy {
    #[default]
    #[serde(rename = "inferred")]
    Inferred,
    #[serde(rename = "exact")]
    Exact,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CensusCountyDivision {
    accuracy: GeocodeAccuracy,
    code: String,
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CensusTract {
    code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreBasedStatArea {
    code: String,
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Place {
    accuracy: GeocodeAccuracy,
    code: String,
    name: String,
    #[serde(rename = "type")]
    type_: PlaceType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaceType {
    #[default]
    #[serde(rename = "unincorporated")]
    Unincorperated,
    #[serde(rename = "incorperated")]
    Incorperated,
}
