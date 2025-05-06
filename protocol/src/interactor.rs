pub struct ServerImpl {
    pub info: domain::entity::server::ServerInfo,
    pub capabilities: domain::entity::server::ServerCapabilities,
    pub tools_repository:
        std::sync::Arc<dyn domain::repository::tool::ToolsRepository + Sync + Send>,
    pub resources_repository:
        std::sync::Arc<dyn domain::repository::resource::ResourcesRepository + Sync + Send>,
}

impl ServerImpl {
    pub fn new(
        tools_repository: impl domain::repository::tool::ToolsRepository + Sync + Send + 'static,
        resources_repository: impl domain::repository::resource::ResourcesRepository
        + Sync
        + Send
        + 'static,
    ) -> Self {
        Self {
            info: domain::entity::server::ServerInfo {
                name: "MCP-RS".to_string(),
                version: "0.1.0".to_string(),
            },
            capabilities: domain::entity::server::ServerCapabilities {
                tools: Some(domain::entity::server::ToolsCapability {
                    list_changed: Some(true),
                }),
                resources: Some(domain::entity::server::ResourcesCapability {
                    subscribe: None,
                    list_changed: Some(true),
                }),
                ..Default::default()
            },
            tools_repository: std::sync::Arc::new(tools_repository),
            resources_repository: std::sync::Arc::new(resources_repository),
        }
    }
}

impl crate::server::Server for ServerImpl {
    fn before_init(
        &self,
        _params: &crate::usecase::lifecycle::InitializeParams,
    ) -> Result<(), jsonrpc::ErrorObject> {
        Ok(())
    }

    fn after_init(&self, _params: &crate::usecase::lifecycle::InitializedParams) {}

    fn info(&self) -> domain::entity::server::ServerInfo {
        self.info.clone()
    }

    fn capabilities(&self) -> domain::entity::server::ServerCapabilities {
        self.capabilities.clone()
    }

    fn instructions(&self) -> Option<String> {
        None
    }

    fn tools_repository(
        &self,
    ) -> std::sync::Arc<dyn domain::repository::tool::ToolsRepository + Sync + Send> {
        self.tools_repository.clone()
    }

    fn resources_repository(
        &self,
    ) -> std::sync::Arc<dyn domain::repository::resource::ResourcesRepository + Sync + Send> {
        self.resources_repository.clone()
    }
}
