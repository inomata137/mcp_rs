pub async fn list_resources(
    server: &(impl protocol::server::Server + Sync),
    req: jsonrpc::Request,
) -> jsonrpc::Response<protocol::usecase::resource::ListResourcesResult> {
    use protocol::usecase::resource::ListResourcesUsecase;

    let output = match req.params.map(serde_json::from_value) {
        Some(Ok(params)) => server
            .list_resources(params)
            .await
            .map_err(jsonrpc::ErrorObject::from),
        Some(Err(err)) => Err(jsonrpc::ErrorObject::parameter_invalid(err)),
        None => Err(jsonrpc::ErrorObject::parameter_missing()),
    };
    jsonrpc::Response::new(req.id, output)
}

pub async fn read_resource(
    server: &(impl protocol::server::Server + Sync),
    req: jsonrpc::Request,
) -> jsonrpc::Response<protocol::usecase::resource::ReadResourceResult> {
    use protocol::usecase::resource::ReadResourceUsecase;

    let output = match req.params.map(serde_json::from_value) {
        Some(Ok(params)) => server
            .read_resource(params)
            .await
            .map_err(jsonrpc::ErrorObject::from),
        Some(Err(err)) => Err(jsonrpc::ErrorObject::parameter_invalid(err)),
        None => Err(jsonrpc::ErrorObject::parameter_missing()),
    };
    jsonrpc::Response::new(req.id, output)
}
