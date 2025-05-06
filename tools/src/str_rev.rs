#[derive(schemars::JsonSchema, serde::Deserialize)]
struct StrRevInput {
    text: String,
}

pub fn info() -> domain::entity::tool::ToolInfo {
    domain::entity::tool::ToolInfo {
        name: "str_rev".to_string(),
        description: Some("Reverses a string".to_string()),
        input_schema: schemars::schema_for!(StrRevInput),
        annotations: None,
    }
}

pub fn call(args: serde_json::Value) -> domain::entity::tool::CallToolResult {
    match serde_json::from_value(args) {
        Ok(StrRevInput { text }) => domain::entity::tool::CallToolResult {
            content: vec![
                domain::entity::tool::TextContent {
                    text: text.chars().rev().collect::<String>(),
                    annotations: None,
                }
                .into(),
            ],
            is_error: None,
        },
        Err(e) => domain::entity::tool::CallToolResult {
            content: vec![
                domain::entity::tool::TextContent {
                    text: e.to_string(),
                    annotations: None,
                }
                .into(),
            ],
            is_error: Some(true),
        },
    }
}
