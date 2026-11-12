use crate::uds_service::UDSService;
use crate::uds_message::UdsRequest;
use crate::common::{*};

pub struct ReadDID;

impl UDSService for ReadDID {
    fn execute_service(&self, req: &UdsRequest, _did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>>{

        if req._payload.len() < 2 {
            println!("Incorrect message length");
            return Err(vec![nrc::NEGATIVE_RESPONSE, nrc::INCORRECT_MESSAGE_LENGTH]);
        }

        println!("Hanling DID {} {}", req._payload[LOW_BYTE_INDEX] , req._payload[HIGH_BYTE_INDEX]);
        let did = u16::from_be_bytes([req._payload[LOW_BYTE_INDEX], req._payload[HIGH_BYTE_INDEX]]);
        if _did_list.contains(&did) && req._sid == service_id::READ_DID_SID {
            println!("Sending positive response for DID {} {}", req._payload[LOW_BYTE_INDEX], req._payload[HIGH_BYTE_INDEX]);
            let response:Vec<u8> = vec![READ_POSITIVE_RESPONSE, req._payload[LOW_BYTE_INDEX], req._payload[HIGH_BYTE_INDEX]];
            return Ok(response);
        }

        println!("Sending Negative response for DID {} {}", req._payload[LOW_BYTE_INDEX], req._payload[HIGH_BYTE_INDEX]);
        Err([nrc::NEGATIVE_RESPONSE, nrc::CONDITIONS_NOT_CORRECT, req._payload[LOW_BYTE_INDEX], req._payload[HIGH_BYTE_INDEX]].to_vec())
    }
}