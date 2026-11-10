use std::{println, vec};

mod uds_server;
use uds_server::{*};
use crate::UDSManager;

fn main() {
    let mut server: UDSManager  = UDSManager::new();
    
    let request = UdsRequest{_sid: 0x22 , _sub_function: 0x01, _payload: vec![0x22, 0xF0 ,0x01]};

    let did_list    = [0xF001,0xF002,0xF003].to_vec();

    let resposnse = server.handle_request(&request, &did_list);

    println!("{:#X?}", resposnse);
}





