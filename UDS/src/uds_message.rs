use tokio::sync::oneshot;


pub struct UdsMessage {
    pub request: UdsRequest,
    pub response_tx: oneshot::Sender<Result<Vec<u8>,Vec<u8>>>
}
#[derive(Debug, Clone)]
pub struct UdsRequest {
    pub _sid: u8,
    pub _payload: Vec<u8>,
}