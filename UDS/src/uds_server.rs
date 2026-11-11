use std::{println, vec};
use crate::UdsRequest;
use crate::uds_service::{*};
use crate::common::*;


pub struct UDSManager {
    pub services: Vec<Box<dyn UDSService>>
}

impl UDSManager {
    pub fn new() -> Self {
        Self { services: vec![Box::new(ReadRequest)] }
    }
    pub fn handle_request(&mut self, req: &UdsRequest, did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>>{

        match req._sid {
            service_id::READ_DID => self.services[service_index::READ_DID_IDX].execute_service(&req, &did_list),
            _ => Err([nrc::UNKOWN, nrc::UNKOWN, nrc::UNKOWN].to_vec())
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