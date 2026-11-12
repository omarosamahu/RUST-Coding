use crate::UdsRequest;

pub trait UDSService: Send + Sync {
    fn execute_service(&self, req: &UdsRequest, _did_list: &Vec<u16>) -> Result<Vec<u8>, Vec<u8>>;
}