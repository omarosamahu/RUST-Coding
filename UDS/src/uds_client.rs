use tokio::sync::{mpsc, oneshot};
use crate::uds_message::{*};


#[derive(Clone)]
pub struct UDSClient {
    pub tx: mpsc::Sender<UdsMessage>
}

impl UDSClient {
    pub async fn send_request(&self, req: UdsRequest) -> Result<Vec<u8>, Vec<u8>>{
        let (response_tx, response_rx) = oneshot::channel();
        let message = UdsMessage{
            request: req, response_tx
        };

        self.tx.send(message).await.unwrap();
        response_rx.await.unwrap()
    }
}