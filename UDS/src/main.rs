use std::{println, vec};

pub mod uds_message;
pub mod uds_server;
pub mod uds_service;
pub mod common;
use uds_server::{*};
use uds_message::UdsRequest;


fn main() {
    let mut server: UDSManager  = UDSManager::new();
    
    let request = UdsRequest{_sid: 0x22, _payload: vec![0xF0, 0x01]};

    let did_list    = [0xF001,0xF002,0xF003].to_vec();

    let resposnse = server.handle_request(&request, &did_list);

    println!("{:#X?}", resposnse);
}





