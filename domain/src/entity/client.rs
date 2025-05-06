#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootsCapability {
    pub list_changed: Option<bool>,
}

#[derive(serde::Deserialize, Default)]
pub struct ClientCapabilities {
    pub experimental: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    pub roots: Option<RootsCapability>,
    pub sampling: Option<serde_json::Value>,
}

#[derive(serde::Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deserialize_capabilities() {
        let json = r#"{
            "experimental": {},
            "roots": {
                "listChanged": true
            },
            "sampling": {}
        }"#;
        let param: Result<super::ClientCapabilities, _> = serde_json::from_str(json);
        assert!(param.is_ok());

        let json = r#"{"sampling":{},"roots":{"listChanged":true}}"#;
        let param: Result<super::ClientCapabilities, _> = serde_json::from_str(json);
        assert!(param.is_ok());
    }

    #[test]
    fn test_deserialize_client_info() {
        let json = r#"{
            "name": "test",
            "version": "1.0"
        }"#;

        let param: Result<super::ClientInfo, _> = serde_json::from_str(json);
        assert!(param.is_ok());
    }
}
