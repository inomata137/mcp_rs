#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "i16", into = "i16")]
pub enum JsonRpcError {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    /// `-32000` to `-32099`
    ServerError(i16),
}

impl JsonRpcError {
    pub fn code(&self) -> i16 {
        match self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::ServerError(code) => *code,
        }
    }

    pub fn from_code(code: i16) -> Result<Self, String> {
        match code {
            -32700 => Ok(Self::ParseError),
            -32600 => Ok(Self::InvalidRequest),
            -32601 => Ok(Self::MethodNotFound),
            -32602 => Ok(Self::InvalidParams),
            -32603 => Ok(Self::InternalError),
            -32099..=-32000 => Ok(Self::ServerError(code)),
            _ => Err(format!("Unexpected error code: {}", code)),
        }
    }
}

impl From<JsonRpcError> for i16 {
    fn from(err: JsonRpcError) -> Self {
        err.code()
    }
}

impl TryFrom<i16> for JsonRpcError {
    type Error = String;
    fn try_from(code: i16) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl From<serde_json::Error> for JsonRpcError {
    fn from(value: serde_json::Error) -> Self {
        match value.classify() {
            serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                Self::ParseError
            }
            serde_json::error::Category::Data => Self::InvalidRequest,
            serde_json::error::Category::Io => Self::InternalError,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ErrorObject {
    pub code: JsonRpcError,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ErrorObject {
    #[inline]
    pub fn parameter_missing() -> Self {
        Self {
            code: JsonRpcError::InvalidParams,
            message: "Parameter missing".into(),
            data: None,
        }
    }

    #[inline]
    pub fn request_id_missing() -> Self {
        Self {
            code: JsonRpcError::InvalidRequest,
            message: "Request ID must not be empty".into(),
            data: None,
        }
    }

    #[inline]
    pub fn parameter_invalid<M: ToString>(message: M) -> Self {
        Self {
            code: JsonRpcError::InvalidParams,
            message: message.to_string(),
            data: None,
        }
    }

    #[inline]
    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: JsonRpcError::MethodNotFound,
            message: format!("Method not found: {}", method),
            data: None,
        }
    }
}

#[cfg(test)]
mod test {
    const ERRORS: [(super::JsonRpcError, &str); 7] = [
        (super::JsonRpcError::ParseError, "-32700"),
        (super::JsonRpcError::InvalidRequest, "-32600"),
        (super::JsonRpcError::MethodNotFound, "-32601"),
        (super::JsonRpcError::InvalidParams, "-32602"),
        (super::JsonRpcError::InternalError, "-32603"),
        (super::JsonRpcError::ServerError(-32000), "-32000"),
        (super::JsonRpcError::ServerError(-32099), "-32099"),
    ];

    #[test]
    fn test_serialize() {
        for (err, expected) in ERRORS {
            let serialized = serde_json::to_string(&err).unwrap();
            assert_eq!(serialized, expected);
        }
    }

    #[test]
    fn test_deserialize() {
        for (expected, err_str) in ERRORS {
            let err: super::JsonRpcError = serde_json::from_str(err_str).unwrap();
            assert_eq!(err, expected);
        }
    }
}
