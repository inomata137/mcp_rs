pub fn ping(
    server: &impl protocol::server::Server,
    req: jsonrpc::Request,
) -> jsonrpc::Response<protocol::usecase::ping::PingResult> {
    use protocol::usecase::ping::PingUsecase;

    let output = match req.params.map(serde_json::from_value) {
        Some(Ok(params)) => server.ping(params),
        Some(Err(err)) => Err(jsonrpc::ErrorObject::parameter_invalid(err)),
        None => Err(jsonrpc::ErrorObject::parameter_missing()),
    };
    jsonrpc::Response::new(req.id, output)
}
