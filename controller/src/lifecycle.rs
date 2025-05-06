pub fn initialize(
    server: &impl protocol::server::Server,
    req: jsonrpc::Request,
) -> jsonrpc::Response<protocol::usecase::lifecycle::InitializeResult> {
    use protocol::usecase::lifecycle::InitializeUsecase;

    let output = match req.params.map(serde_json::from_value) {
        Some(Ok(params)) => server.initialize(params),
        Some(Err(err)) => Err(jsonrpc::ErrorObject::parameter_invalid(err)),
        None => Err(jsonrpc::ErrorObject::parameter_missing()),
    };
    jsonrpc::Response::new(req.id, output)
}

pub fn initialized(server: &impl protocol::server::Server, req: jsonrpc::Request) {
    use protocol::usecase::lifecycle::InitializedUsecase;

    if let Some(Ok(params)) = req.params.map(serde_json::from_value) {
        server.initialized(params)
    }
}
