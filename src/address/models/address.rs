use super::{Metadata, Geometry};

#[derive(
    Deserialize, Debug, Clone
)]
pub struct Address {
    #[serde(rename = "type")]
    pub category: String,
    pub properties: Metadata,
    pub geometry: Geometry
}