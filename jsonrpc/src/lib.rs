mod error;
mod id;
mod version;

pub use error::{ErrorObject, JsonRpcError};
pub use id::RequestId;
pub use version::JsonRpcVersion;

#[derive(serde::Deserialize)]
pub struct Request {
    pub jsonrpc: JsonRpcVersion,
    pub id: Option<RequestId>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(serde::Serialize)]
#[serde(remote = "Result")]
enum Output<T, E> {
    #[serde(rename = "result")]
    Ok(T),
    #[serde(rename = "error")]
    Err(E),
}

#[derive(serde::Serialize)]
pub struct Response<T>
where
    T: serde::Serialize,
{
    pub jsonrpc: JsonRpcVersion,
    pub id: Option<RequestId>,
    #[serde(flatten)]
    #[serde(with = "Output")]
    pub output: Result<T, error::ErrorObject>,
}

impl<T> Response<T>
where
    T: serde::Serialize,
{
    #[inline]
    pub fn new(id: Option<RequestId>, output: Result<T, error::ErrorObject>) -> Self {
        Self {
            jsonrpc: Default::default(),
            id,
            output,
        }
    }

    pub fn into_json(self) -> Response<serde_json::Value> {
        let output = match self.output.map(serde_json::to_value) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(err)) => Err(ErrorObject {
                code: JsonRpcError::InternalError,
                message: err.to_string(),
                data: None,
            }),
            Err(err) => Err(err),
        };
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id,
            output,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Batchable<T> {
    Single(T),
    Batch(Vec<T>),
}

impl<T: serde::Serialize> From<Response<T>> for Batchable<Response<serde_json::Value>> {
    fn from(value: Response<T>) -> Self {
        Batchable::Single(value.into_json())
    }
}

impl<T: serde::Serialize> From<Vec<Response<T>>> for Batchable<Response<serde_json::Value>> {
    fn from(value: Vec<Response<T>>) -> Self {
        Batchable::Batch(value.into_iter().map(|v| v.into_json()).collect())
    }
}

pub type BatchableRequest = Batchable<Request>;

pub type BatchableResponse = Batchable<Response<serde_json::Value>>;

#[cfg(test)]
mod test {
    #[test]
    fn test_serialize_ok() {
        let res: super::Response<usize> = super::Response {
            jsonrpc: super::JsonRpcVersion::V2_0,
            id: Some(super::RequestId::Number(1)),
            output: Ok(42),
        };
        let serialized = serde_json::to_string(&res).unwrap();
        let expected = r#"{"jsonrpc":"2.0","id":1,"result":42}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_serialize_err() {
        let res: super::Response<()> = super::Response {
            jsonrpc: super::JsonRpcVersion::V2_0,
            id: Some(super::RequestId::Number(1)),
            output: Err(super::error::ErrorObject {
                code: super::JsonRpcError::InvalidParams,
                message: "error".to_string(),
                data: None,
            }),
        };
        let serialized = serde_json::to_string(&res).unwrap();
        let expected = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32602,"message":"error"}}"#;
        assert_eq!(serialized, expected);
    }
}
