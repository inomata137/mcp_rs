#[derive(serde::Deserialize)]
pub struct PingParams {}

#[derive(serde::Serialize)]
pub struct PingResult {}

pub trait PingUsecase {
    fn ping(&self, params: PingParams) -> Result<PingResult, jsonrpc::ErrorObject>;
}
