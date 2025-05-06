pub fn server() -> impl protocol::server::Server {
    let tools_repository = tools::ToolsRepositoryImpl::new();
    let resources_repository = resources::ResourcesRepositoryImpl::new();
    protocol::interactor::ServerImpl::new(tools_repository, resources_repository)
}
