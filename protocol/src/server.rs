pub trait Server {
    fn before_init(
        &self,
        params: &crate::usecase::lifecycle::InitializeParams,
    ) -> Result<(), jsonrpc::ErrorObject>;

    fn after_init(&self, params: &crate::usecase::lifecycle::InitializedParams);

    fn info(&self) -> domain::entity::server::ServerInfo;

    fn capabilities(&self) -> domain::entity::server::ServerCapabilities;

    fn instructions(&self) -> Option<String>;

    fn tools_repository(
        &self,
    ) -> std::sync::Arc<dyn domain::repository::tool::ToolsRepository + Sync + Send>;

    fn resources_repository(
        &self,
    ) -> std::sync::Arc<dyn domain::repository::resource::ResourcesRepository + Sync + Send>;
}

impl<S: Server> crate::usecase::lifecycle::InitializeUsecase for S {
    fn initialize(
        &self,
        params: crate::usecase::lifecycle::InitializeParams,
    ) -> Result<crate::usecase::lifecycle::InitializeResult, jsonrpc::ErrorObject> {
        self.before_init(&params)?;
        Ok(crate::usecase::lifecycle::InitializeResult {
            protocol_version: params.protocol_version,
            capabilities: self.capabilities(),
            server_info: self.info(),
            instructions: self.instructions(),
        })
    }
}

impl<S: Server> crate::usecase::lifecycle::InitializedUsecase for S {
    fn initialized(&self, params: crate::usecase::lifecycle::InitializedParams) {
        self.after_init(&params);
    }
}

impl<S: Server> crate::usecase::ping::PingUsecase for S {
    fn ping(
        &self,
        _params: crate::usecase::ping::PingParams,
    ) -> Result<crate::usecase::ping::PingResult, jsonrpc::ErrorObject> {
        Ok(crate::usecase::ping::PingResult {})
    }
}

impl<S: Server> crate::usecase::tool::ListToolsUsecase for S {
    fn list_tools(
        &self,
        params: Option<crate::usecase::tool::ListToolsParams>,
    ) -> Result<crate::usecase::tool::ListToolsResult, crate::usecase::tool::ListToolsError> {
        self.tools_repository()
            .list(params.and_then(|p| p.cursor))
            .map_err(crate::usecase::tool::ListToolsError::from)
    }
}

#[async_trait::async_trait]
impl<S: Server + Sync> crate::usecase::tool::CallToolUsecase for S {
    async fn call_tool(
        &self,
        params: crate::usecase::tool::CallToolParams,
    ) -> Result<crate::usecase::tool::CallToolResult, crate::usecase::tool::CallToolError> {
        self.tools_repository()
            .call(&params.name, params.arguments)
            .await
            .map_err(crate::usecase::tool::CallToolError::from)
    }
}

#[async_trait::async_trait]
impl<S: Server + Sync> crate::usecase::resource::ListResourcesUsecase for S {
    async fn list_resources(
        &self,
        params: Option<crate::usecase::resource::ListResourcesParams>,
    ) -> Result<
        crate::usecase::resource::ListResourcesResult,
        crate::usecase::resource::ListResourcesError,
    > {
        self.resources_repository()
            .list(params.and_then(|p| p.cursor))
            .await
            .map_err(crate::usecase::resource::ListResourcesError::from)
    }
}

#[async_trait::async_trait]
impl<S: Server + Sync> crate::usecase::resource::ReadResourceUsecase for S {
    async fn read_resource(
        &self,
        params: crate::usecase::resource::ReadResourceParams,
    ) -> Result<
        crate::usecase::resource::ReadResourceResult,
        crate::usecase::resource::ReadResourceError,
    > {
        self.resources_repository()
            .read(&params.uri)
            .await
            .map_err(crate::usecase::resource::ReadResourceError::from)
    }
}
