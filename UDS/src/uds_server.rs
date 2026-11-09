use core::sync;
use std::{println, vec};

#[allow(dead_code)]
pub trait UDSService {
    fn execute_service(&mut self, req: &UdsRequest);
    fn get_response(&self) -> UdsResponse;
}

struct ReadRequest {
    _did: u8,
    _did_list : Vec<u8>,
    _response: UdsResponse
}

impl UDSService for ReadRequest {

    fn execute_service(&mut self, req: &UdsRequest){
        println!("Hanling DID {}", req._payload[1]);
        self.read_did(req._payload[1]);
    }
    fn get_response(&self) -> UdsResponse {
        self._response.clone()
    }
}

impl ReadRequest {

    fn new() -> Self {
        Self { _did: 0x0, _did_list: vec![0x01, 0x02, 0x03], _response: UdsResponse {
            request_type : String::from("") , payload: Vec::new()
        }  }
    } 
    fn read_did(&mut self, did: u8){        
        
        if self._did_list.contains(&did) {
            self._response = UdsResponse { request_type : String::from("Positive response for Read Request") , payload: vec![0x62, self._did]};
        }
    }
}

pub struct UDSManager {
    pub services: Vec<Box<dyn UDSService>>
}

impl UDSManager {
    pub fn new() -> Self {
        Self { services: vec![Box::new(ReadRequest::new())] }
    }
    pub fn handle_request(&mut self, req: &UdsRequest) -> UdsResponse{
        if req._sid == 0x22 {   
            let  read_request    = &mut self.services[0];
            read_request.execute_service(&req);
            return read_request.get_response();
        }

        UdsResponse { request_type: String::from("Unsupported type"), payload: vec![0xFF, 0xFF, 0xFF] }
    }
}

#[derive(Debug, Clone)]
pub struct UdsRequest {
    pub _sid: u8,
    pub _sub_function: u16,
    pub _payload: Vec<u8>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UdsResponse {
    pub request_type: String,
    pub payload: Vec<u8>
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