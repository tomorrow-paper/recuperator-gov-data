#[derive(
    Deserialize,
    Debug, Clone
)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub category: String,
    pub coordinates: Vec<f32>
}