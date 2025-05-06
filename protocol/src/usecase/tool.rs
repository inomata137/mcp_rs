#[derive(serde::Deserialize)]
pub struct ListToolsParams {
    pub cursor: Option<String>,
}

pub use domain::entity::tool::ListToolsResult;

pub struct ListToolsError;

impl From<domain::entity::tool::ToolError> for ListToolsError {
    fn from(value: domain::entity::tool::ToolError) -> Self {
        match value {
            domain::entity::tool::ToolError::InternalError => ListToolsError,
            _ => unreachable!(),
        }
    }
}

impl From<ListToolsError> for jsonrpc::ErrorObject {
    fn from(_value: ListToolsError) -> Self {
        jsonrpc::ErrorObject {
            code: jsonrpc::JsonRpcError::InternalError,
            message: "Internal error".into(),
            data: None,
        }
    }
}

pub trait ListToolsUsecase {
    fn list_tools(
        &self,
        params: Option<ListToolsParams>,
    ) -> Result<ListToolsResult, ListToolsError>;
}

#[derive(serde::Deserialize)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: Option<serde_json::Value>,
}

pub use domain::entity::tool::CallToolResult;

pub enum CallToolError {
    ParameterMissing,
    UnknownTool(String),
    InternalError,
}

impl From<domain::entity::tool::ToolError> for CallToolError {
    fn from(value: domain::entity::tool::ToolError) -> Self {
        match value {
            domain::entity::tool::ToolError::ParameterMissing => CallToolError::ParameterMissing,
            domain::entity::tool::ToolError::UnknownTool(tool) => CallToolError::UnknownTool(tool),
            domain::entity::tool::ToolError::InternalError => CallToolError::InternalError,
            _ => CallToolError::InternalError,
        }
    }
}

impl From<CallToolError> for jsonrpc::ErrorObject {
    fn from(value: CallToolError) -> Self {
        match value {
            CallToolError::ParameterMissing => jsonrpc::ErrorObject {
                code: jsonrpc::JsonRpcError::InvalidParams,
                message: "Parameter missing".into(),
                data: None,
            },
            CallToolError::UnknownTool(tool) => jsonrpc::ErrorObject {
                code: jsonrpc::JsonRpcError::InvalidParams,
                message: format!("Unknown tool: {}", tool),
                data: None,
            },
            CallToolError::InternalError => jsonrpc::ErrorObject {
                code: jsonrpc::JsonRpcError::InternalError,
                message: "Internal error".into(),
                data: None,
            },
        }
    }
}

#[async_trait::async_trait]
pub trait CallToolUsecase {
    async fn call_tool(&self, params: CallToolParams) -> Result<CallToolResult, CallToolError>;
}
