#[derive(serde::Serialize)]
pub struct TextContent {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<super::annotation::Annotations>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
    pub data: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<super::annotation::Annotations>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioContent {
    pub data: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<super::annotation::Annotations>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Content {
    Text(TextContent),
    Image(ImageContent),
    Audio(AudioContent),
    // Resource,
}

impl From<TextContent> for Content {
    fn from(content: TextContent) -> Self {
        Content::Text(content)
    }
}

impl From<ImageContent> for Content {
    fn from(content: ImageContent) -> Self {
        Content::Image(content)
    }
}

impl From<AudioContent> for Content {
    fn from(content: AudioContent) -> Self {
        Content::Audio(content)
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destructive_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idempotent_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_world_hint: Option<bool>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: schemars::schema::RootSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,
}

#[derive(serde::Serialize)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResult {
    pub tools: Vec<ToolInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[non_exhaustive]
pub enum ToolError {
    ParameterMissing,
    UnknownTool(String),
    InternalError,
}
