pub mod nrc {
    pub const CONDITIONS_NOT_CORRECT: u8 = 0x22;
    pub const NEGATIVE_RESPONSE: u8 = 0x7F;
    pub const INCORRECT_MESSAGE_LENGTH: u8 = 0x13;
    pub const SERVICE_NOT_SUPPORTED: u8 = 0x11;
    pub const UNKOWN: u8 = 0xFF;
}

pub mod service_id {
    pub const READ_DID_SID: u8 = 0x22;
    pub const DIAGNOSTICS_SESSION_CTL_SID: u8 = 0x10;
}

pub mod service_index {
    pub const READ_DID_IDX: usize = 0;
    pub const DIAGNOSTICS_SESSION_CTL_IDX: usize = 1;
}

pub const READ_POSITIVE_RESPONSE: u8 = 0x62;
pub const LOW_BYTE_INDEX: usize = 0;
pub const HIGH_BYTE_INDEX: usize = 1;

pub mod session_control {
    pub const SESSION_CTL_POSITIVSE_RSPONSE: u8 = 0x50;
    pub const DEFAULT_SESSION: u8 = 0x01;
    pub const PROGRAMMING_SESSION: u8 = 0x02;
    pub const EXTENDED_SESSION: u8 = 0x03;
}