use std::sync::Arc;
use std::vec;
use crate::uds_service::{*};
use crate::common::*;
use crate::uds_message::UdsMessage;
use tokio::sync::mpsc;
pub struct UDSServer {
    rx: mpsc::Receiver<UdsMessage>,
    manager: Arc<UDSManager>,
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
    pub services: Vec<Box<dyn UDSService>>
}

impl UDSManager {
    pub fn new() -> Self {
        Self { services: vec![Box::new(ReadRequest)] }
    }
    pub fn handle_request(&self, req: &UdsMessage) -> Result<Vec<u8>, Vec<u8>>{
        let did_list    = [0xF001,0xF002,0xF003].to_vec();
        match req.request._sid {
            service_id::READ_DID => self.services[service_index::READ_DID_IDX].execute_service(&req.request, &did_list),
            _ => Err([nrc::SERVICE_NOT_SUPPORTED].to_vec())
        }
    }
}


/*
    type UDSResponseHandler = Box<dyn Fn(UdsRequest)>;


    impl UDSManager {
        pub fn handle_uds_request(req: &UdsRequest){

        }
    }

    type Handler = Box<dyn Fn(&UdsRequest) + Send + Sync>;
    pub struct uds_server {
        handler: Option<Handler>,
        requests: Vec<Option<Handler>>
    }

    #[allow(dead_code)]
    impl uds_server {
        pub fn new() -> Self {
            println!("Initlizing DOIP Server");
            Self {
                handler: None,
                requests: Vec::new(),
            }
        }

        pub fn register_callback<F> (&mut self, callback: F) 
        where 
            F: Fn(&UdsRequest) + Send + Sync + 'static,
        {
            self.requests.push(Some(Box::new(callback)));
        }

        fn execute_request(&self, req: &UdsRequest) {

        }
    }
*/