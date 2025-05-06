#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInfo {
    pub uri: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<super::annotation::Annotations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceData {
    Text(String),
    Blob(String),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceContent {
    pub uri: String,
    pub mime_type: Option<String>,
    #[serde(flatten)]
    pub data: crate::entity::resource::ResourceData,
}

#[derive(serde::Serialize)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContent>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesResult {
    pub resources: Vec<crate::entity::resource::ResourceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[non_exhaustive]
pub enum ResourceError {
    NotFound(String),
    InvalidUri(String),
    InternalError,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_serialize_text() {
        let text_resource = super::ResourceContent {
            uri: "test://foo_text".into(),
            mime_type: Some("text/plain".into()),
            data: super::ResourceData::Text("Hello world!".into()),
        };
        let serialized = serde_json::to_string(&text_resource).unwrap();
        let expected = r#"{"uri":"test://foo_text","mimeType":"text/plain","text":"Hello world!"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_serialize_blob() {
        let text_resource = super::ResourceContent {
            uri: "test://foo_blob".into(),
            mime_type: Some("application/octet-stream".into()),
            data: super::ResourceData::Blob("SGVsbG8gd29ybGQh".into()),
        };
        let serialized = serde_json::to_string(&text_resource).unwrap();
        let expected = r#"{"uri":"test://foo_blob","mimeType":"application/octet-stream","blob":"SGVsbG8gd29ybGQh"}"#;
        assert_eq!(serialized, expected);
    }
}
