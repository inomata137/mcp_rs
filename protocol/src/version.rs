#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum ProtocolVersion {
    #[default]
    #[serde(rename = "2024-11-05")]
    V2024_11_05,
    #[serde(rename = "2025-03-26")]
    V2025_03_26,
}
