use super::Address;

#[derive(
    Deserialize, Debug, Clone
)]
pub struct Record {
    pub limit: i8,
    pub query: String,
    pub features: Vec<Address>,
    pub version: String,
    #[serde(rename = "type")]
    pub category: String,
    pub licence: String,
    pub attribution: String
}