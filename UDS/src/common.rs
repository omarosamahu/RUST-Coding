pub mod nrc {
    pub const CONDITIONS_NOT_CORRECT: u8 = 0x22;
    pub const NEGATIVS_RESPONSE: u8 = 0x7F;
    pub const INCORRECT_MESSAGE_LENGTH: u8 = 0x13;
    pub const SERVICE_NOT_SUPPORTED: u8 = 0x11;
    pub const UNKOWN: u8 = 0xFF;
}


pub mod service_id {
    pub const READ_DID: u8 = 0x22;
}

pub mod service_index {
    pub const READ_DID_IDX: usize = 0;
}