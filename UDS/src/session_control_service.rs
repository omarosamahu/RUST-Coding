use crate::uds_service::UDSService;
use crate::uds_message::UdsRequest;
use crate::common::{*};

pub struct DiagnosticsSessionControl;

impl UDSService for DiagnosticsSessionControl{
    fn execute_service(&self, req: &UdsRequest, _did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>> {
        if req._payload.len() < 1 {
            println!("Incorrect message length");
            return Err(vec![nrc::NEGATIVE_RESPONSE, nrc::INCORRECT_MESSAGE_LENGTH]);
        }

        if req._sid == service_id::DIAGNOSTICS_SESSION_CTL_SID {
            return match req._payload[LOW_BYTE_INDEX] {
                session_control::DEFAULT_SESSION => {
                    println!("Entering default session");
                    Ok([session_control::SESSION_CTL_POSITIVSE_RSPONSE,session_control::DEFAULT_SESSION].to_vec())},
                session_control::EXTENDED_SESSION => {
                    println!("Entering programming session");
                    Ok([session_control::SESSION_CTL_POSITIVSE_RSPONSE,session_control::EXTENDED_SESSION].to_vec())},
                session_control::PROGRAMMING_SESSION => {
                    println!("Entering extended session");
                    Ok([session_control::SESSION_CTL_POSITIVSE_RSPONSE,session_control::PROGRAMMING_SESSION].to_vec())},
                _ => Err([nrc::UNKOWN, nrc::UNKOWN].to_vec()),
            };
        }

        Err([nrc::NEGATIVE_RESPONSE, nrc::CONDITIONS_NOT_CORRECT].to_vec())
    }
}