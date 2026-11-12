use std::{println, vec};

pub mod uds_message;
pub mod uds_server;
pub mod uds_service;
pub mod common;
pub mod uds_client;
use uds_server::{*};
use uds_message::{*};

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::uds_client::UDSClient;

#[tokio::main]
async fn main() {
        let (tx, rx) = mpsc::channel::<UdsMessage>(32);
        let manager = Arc::new(UDSManager::new());

        let server = UDSServer{
            rx,
            manager
        };

        let server_handle = tokio::spawn(
            async move {server.run().await}
        );

        let client = UDSClient{tx};
        
        let client1 = client.clone();
        let res = client1.send_request(UdsRequest { _sid: 0x22, _payload: vec![0xF0,0x01] }).await;
        match res {
            Ok(data) => println!("Got positive response {:#X?}", data),
            Err(nrc) => println!("Got Negative response {:#X?}", nrc),
        };

        let client2 = client.clone();
        let res = client2.send_request(UdsRequest { _sid: 0x22, _payload: vec![0xF] }).await;
        match res {
            Ok(data) => println!("Got positive response {:#X?}", data),
            Err(nrc) => println!("Got Negative response {:#X?}", nrc),
        };

        let client3 = client.clone();
        let res = client3.send_request(UdsRequest { _sid: 0x22, _payload: vec![0xF0,0x11] }).await;
        match res {
            Ok(data) => println!("Got positive response {:#X?}", data),
            Err(nrc) => println!("Got Negative response {:#X?}", nrc),
        };

        let _ = server_handle.await;

}