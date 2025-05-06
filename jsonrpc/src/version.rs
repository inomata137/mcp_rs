#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum JsonRpcVersion {
    #[default]
    #[serde(rename = "2.0")]
    V2_0,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_serialize() {
        let v = super::JsonRpcVersion::V2_0;
        let serialized = serde_json::to_string(&v).unwrap();
        let expected = r#""2.0""#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize() {
        let json = r#""2.0""#;
        let deserialized: super::JsonRpcVersion = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, super::JsonRpcVersion::V2_0);
    }
}
