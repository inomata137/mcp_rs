pub fn list_tools(
    server: &impl protocol::server::Server,
    req: jsonrpc::Request,
) -> jsonrpc::Response<protocol::usecase::tool::ListToolsResult> {
    use protocol::usecase::tool::ListToolsUsecase;

    let output = match req.params.map(serde_json::from_value).transpose() {
        Ok(params) => server
            .list_tools(params)
            .map_err(jsonrpc::ErrorObject::from),
        Err(err) => Err(jsonrpc::ErrorObject::parameter_invalid(err)),
    };
    jsonrpc::Response::new(req.id, output)
}

pub async fn call_tool(
    server: &(impl protocol::server::Server + Sync),
    req: jsonrpc::Request,
) -> jsonrpc::Response<protocol::usecase::tool::CallToolResult> {
    use protocol::usecase::tool::CallToolUsecase;

    let output = match req.params.map(serde_json::from_value) {
        Some(Ok(params)) => server
            .call_tool(params)
            .await
            .map_err(jsonrpc::ErrorObject::from),
        Some(Err(err)) => Err(jsonrpc::ErrorObject::parameter_invalid(err)),
        None => Err(jsonrpc::ErrorObject::parameter_missing()),
    };
    jsonrpc::Response::new(req.id, output)
}
