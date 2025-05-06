mod lifecycle;
mod ping;
mod resource;
mod tool;

async fn handle_single_request(
    server: &(impl protocol::server::Server + Send + Sync),
    req: jsonrpc::Request,
) -> Option<jsonrpc::Response<serde_json::Value>> {
    match req.method.as_str() {
        "initialize" => Some(lifecycle::initialize(server, req).into_json()),
        "notifications/initialized" => {
            lifecycle::initialized(server, req);
            None
        }
        "tools/list" => Some(tool::list_tools(server, req).into_json()),
        "tools/call" => Some(tool::call_tool(server, req).await.into_json()),
        "resources/list" => Some(resource::list_resources(server, req).await.into_json()),
        "resources/read" => Some(resource::read_resource(server, req).await.into_json()),
        "ping" => Some(ping::ping(server, req).into_json()),
        _ => Some(jsonrpc::Response::<serde_json::Value>::new(
            req.id,
            Err(jsonrpc::ErrorObject::method_not_found(&req.method)),
        )),
    }
}

pub async fn handle_batchable_request(
    server: &(impl protocol::server::Server + Send + Sync),
    reqs: jsonrpc::BatchableRequest,
) -> Option<jsonrpc::BatchableResponse> {
    match reqs {
        jsonrpc::BatchableRequest::Single(req) => {
            handle_single_request(server, req).await.map(Into::into)
        }
        jsonrpc::BatchableRequest::Batch(reqs) => {
            let mut responses = Vec::new();
            for req in reqs {
                if let Some(res) = handle_single_request(server, req).await {
                    responses.push(res);
                }
            }
            Some(responses.into())
        }
    }
}
