use std::{println, vec};

mod uds_server;
use uds_server::{*};

// mod uds_server;
fn main() {
    let mut server: UDSManager  = UDSManager::new();
    
    let request = UdsRequest{_sid: 0x22 , _sub_function: 0x01, _payload: vec![0x22, 0x01]};

    let resposnse = server.handle_request(&request);

    println!("{:#X?}", resposnse);
}





