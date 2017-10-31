#[derive(
    Deserialize, Debug, Clone
)]
pub struct Metadata {
    pub id: String,
    #[serde(rename = "type")]
    pub category: String,
    pub importance: f32,
    pub postcode: String,
    pub score: f32,
    pub label: String,
    pub city: String,
    pub citycode: String,
    pub name: String,
    pub context: String,
    pub x: f32,
    pub y: f32
}