mod di;
mod logger;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    logger::init_logger().expect("Failed to initialize logger");
    let mut transport = transport::Stdio::new();
    let server = di::server();

    loop {
        let req = transport.receive().await?;
        let res = controller::handle_batchable_request(&server, req).await;
        if let Some(res) = res {
            transport.send(res).await?;
        }
    }
}
