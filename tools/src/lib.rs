mod str_rev;

#[derive(Default)]
pub struct ToolsRepositoryImpl;

impl ToolsRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl domain::repository::tool::ToolsRepository for ToolsRepositoryImpl {
    async fn call(
        &self,
        name: &str,
        args: Option<serde_json::Value>,
    ) -> Result<domain::entity::tool::CallToolResult, domain::entity::tool::ToolError> {
        match name {
            "str_rev" => match args {
                Some(args) => Ok(str_rev::call(args)),
                None => Err(domain::entity::tool::ToolError::ParameterMissing),
            },
            _ => Err(domain::entity::tool::ToolError::UnknownTool(name.into())),
        }
    }

    fn list(
        &self,
        _cursor: Option<String>,
    ) -> Result<domain::entity::tool::ListToolsResult, domain::entity::tool::ToolError> {
        Ok(domain::entity::tool::ListToolsResult {
            tools: vec![str_rev::info()],
            next_cursor: None,
        })
    }
}
