#[derive(serde::Deserialize)]
pub struct ListResourcesParams {
    pub cursor: Option<String>,
}

pub use domain::entity::resource::ListResourcesResult;

pub struct ListResourcesError;

impl From<domain::entity::resource::ResourceError> for ListResourcesError {
    fn from(value: domain::entity::resource::ResourceError) -> Self {
        match value {
            domain::entity::resource::ResourceError::InternalError => ListResourcesError,
            _ => unreachable!(),
        }
    }
}

impl From<ListResourcesError> for jsonrpc::ErrorObject {
    fn from(_value: ListResourcesError) -> Self {
        jsonrpc::ErrorObject {
            code: jsonrpc::JsonRpcError::InternalError,
            message: "Internal error".into(),
            data: None,
        }
    }
}

#[async_trait::async_trait]
pub trait ListResourcesUsecase {
    async fn list_resources(
        &self,
        params: Option<ListResourcesParams>,
    ) -> Result<ListResourcesResult, ListResourcesError>;
}

#[derive(serde::Deserialize)]
pub struct ReadResourceParams {
    pub uri: String,
}

pub use domain::entity::resource::ReadResourceResult;

pub enum ReadResourceError {
    NotFound(String),
    InvalidUri(String),
    InternalError,
}

impl From<domain::entity::resource::ResourceError> for ReadResourceError {
    fn from(value: domain::entity::resource::ResourceError) -> Self {
        match value {
            domain::entity::resource::ResourceError::NotFound(uri) => {
                ReadResourceError::NotFound(uri)
            }
            domain::entity::resource::ResourceError::InvalidUri(uri) => {
                ReadResourceError::InvalidUri(uri)
            }
            domain::entity::resource::ResourceError::InternalError => {
                ReadResourceError::InternalError
            }
            _ => unreachable!(),
        }
    }
}

impl From<ReadResourceError> for jsonrpc::ErrorObject {
    fn from(value: ReadResourceError) -> Self {
        match value {
            ReadResourceError::NotFound(uri) => jsonrpc::ErrorObject {
                code: jsonrpc::JsonRpcError::InvalidParams,
                message: format!("Resource not found: {}", uri),
                data: None,
            },
            ReadResourceError::InvalidUri(uri) => jsonrpc::ErrorObject {
                code: jsonrpc::JsonRpcError::InvalidParams,
                message: format!("Invalid URI: {}", uri),
                data: None,
            },
            ReadResourceError::InternalError => jsonrpc::ErrorObject {
                code: jsonrpc::JsonRpcError::InternalError,
                message: "Internal error".into(),
                data: None,
            },
        }
    }
}

#[async_trait::async_trait]
pub trait ReadResourceUsecase {
    async fn read_resource(
        &self,
        params: ReadResourceParams,
    ) -> Result<ReadResourceResult, ReadResourceError>;
}
