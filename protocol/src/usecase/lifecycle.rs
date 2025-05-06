#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub protocol_version: crate::version::ProtocolVersion,
    pub capabilities: domain::entity::client::ClientCapabilities,
    pub client_info: domain::entity::client::ClientInfo,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub protocol_version: crate::version::ProtocolVersion,
    pub capabilities: domain::entity::server::ServerCapabilities,
    pub server_info: domain::entity::server::ServerInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

pub trait InitializeUsecase {
    fn initialize(
        &self,
        params: InitializeParams,
    ) -> Result<InitializeResult, jsonrpc::ErrorObject>;
}

#[derive(serde::Deserialize)]
pub struct InitializedParams {}

pub trait InitializedUsecase {
    fn initialized(&self, params: InitializedParams);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deserialize() {
        let json = r#"{
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test",
                "version": "0.1.0"
            }
        }"#;
        let param: Result<super::InitializeParams, _> = serde_json::from_str(json);
        assert!(param.is_ok());

        let json = r#"{"protocolVersion":"2024-11-05","capabilities":{"sampling":{},"roots":{"listChanged":true}},"clientInfo":{"name":"mcp-inspector","version":"0.11.0"}}"#;
        let param: Result<super::InitializeParams, _> = serde_json::from_str(json);
        assert!(param.is_ok());
    }
}
