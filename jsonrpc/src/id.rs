#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(usize),
}
