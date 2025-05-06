#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Priority {
    Optional = 0,
    Required = 1,
}

#[derive(serde::Serialize)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,
}
