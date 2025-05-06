mod pi;

#[derive(Default)]
pub struct ResourcesRepositoryImpl;

impl ResourcesRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl domain::repository::resource::ResourcesRepository for ResourcesRepositoryImpl {
    async fn list(
        &self,
        _cursor: Option<String>,
    ) -> Result<
        domain::entity::resource::ListResourcesResult,
        domain::entity::resource::ResourceError,
    > {
        Ok(domain::entity::resource::ListResourcesResult {
            resources: vec![pi::info()?],
            next_cursor: None,
        })
    }

    async fn read(
        &self,
        uri: &str,
    ) -> Result<domain::entity::resource::ReadResourceResult, domain::entity::resource::ResourceError>
    {
        match uri {
            pi::URI => pi::read().await,
            _ => Err(domain::entity::resource::ResourceError::NotFound(
                uri.into(),
            )),
        }
    }
}
