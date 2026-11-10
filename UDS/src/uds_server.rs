use std::{println, vec};

pub mod nrc {
    pub const CONDITIONS_NOT_CORRECT: u8 = 0x22;
}

pub mod service_id {
    pub const READ_DID: u8 = 0x22;
}

#[allow(dead_code)]
pub trait UDSService {
    fn execute_service(&mut self, req: &UdsRequest, _did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>>;
}

pub struct ReadRequest;

impl UDSService for ReadRequest {

    fn execute_service(&mut self, req: &UdsRequest, _did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>>{
        println!("Hanling DID {}", req._payload[1]);
        let did = u16::from_be_bytes([req._payload[1], req._payload[2]]);
        if _did_list.contains(&did) {
            println!("Sending positive response for DID {}", req._payload[1]);
            let response:Vec<u8> = vec![0x62, req._payload[1], req._payload[2]];
            return Ok(response);
        }

        println!("Sending Negative response for DID {} {}", req._payload[1], req._payload[2]);
        Err([0x71, nrc::CONDITIONS_NOT_CORRECT, req._payload[1], req._payload[2]].to_vec())
    }
}

pub struct UDSManager {
    pub services: Vec<Box<dyn UDSService>>
}

impl UDSManager {
    pub fn new() -> Self {
        Self { services: vec![Box::new(ReadRequest)] }
    }
    pub fn handle_request(&mut self, req: &UdsRequest, did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>>{
        if req._sid == service_id::READ_DID {   
            let  read_request    = &mut self.services[0];
            return read_request.execute_service(&req, &did_list);
        }

        Err([0xFF,0xFF,0xFF].to_vec())
    }
}

#[derive(Debug, Clone)]
pub struct UdsRequest {
    pub _sid: u8,
    pub _sub_function: u16,
    pub _payload: Vec<u8>,
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