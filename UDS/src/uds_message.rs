#[derive(Debug, Clone)]
pub struct UdsRequest {
    pub _sid: u8,
    pub _payload: Vec<u8>,
}