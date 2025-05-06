#[async_trait::async_trait]
pub trait ToolsRepository {
    async fn call(
        &self,
        name: &str,
        args: Option<serde_json::Value>,
    ) -> Result<crate::entity::tool::CallToolResult, crate::entity::tool::ToolError>;

    fn list(
        &self,
        cursor: Option<String>,
    ) -> Result<crate::entity::tool::ListToolsResult, crate::entity::tool::ToolError>;
}
