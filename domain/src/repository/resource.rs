#[async_trait::async_trait]
pub trait ResourcesRepository {
    async fn list(
        &self,
        cursor: Option<String>,
    ) -> Result<crate::entity::resource::ListResourcesResult, crate::entity::resource::ResourceError>;

    async fn read(
        &self,
        uri: &str,
    ) -> Result<crate::entity::resource::ReadResourceResult, crate::entity::resource::ResourceError>;
}
