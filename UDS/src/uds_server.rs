use std::sync::Arc;
use std::vec;
use crate::uds_service::{*};
use crate::common::*;
use crate::uds_message::UdsMessage;
use tokio::sync::mpsc;

pub struct UDSServer {
    pub rx: mpsc::Receiver<UdsMessage>,
    pub manager: Arc<UDSManager>,
}

impl UDSServer {
    pub async fn run(mut self) {
        while let Some(message) = self.rx.recv().await {
            let manager = Arc::clone(&self.manager);
            tokio::spawn(async move {
                let response = manager.handle_request(&message);
                let _ = message.response_tx.send(response);
            });
        }
    }
}
pub struct UDSManager {
    pub services: Vec<Box<dyn UDSService>>,
    did_list: Vec<u16> // TODO: This needs to be parsed from config file 
}

impl UDSManager {
    pub fn new() -> Self {
        Self { services: vec![Box::new(ReadRequest)], did_list: [0xF001,0xF002,0xF003].to_vec() }
    }
    pub fn handle_request(&self, req: &UdsMessage) -> Result<Vec<u8>, Vec<u8>>{
        match req.request._sid {
            service_id::READ_DID => self.services[service_index::READ_DID_IDX].execute_service(&req.request, &self.did_list),
            _ => Err([nrc::SERVICE_NOT_SUPPORTED].to_vec())
        }
    }
}