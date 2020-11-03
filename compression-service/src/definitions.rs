// The magic number present at the beginning of each message.
// TODO Reset this
pub const MAGIC_NUMBER: u32 = 0x53545259;
// The (user-defined) maximum payload size.
pub const MAXIMUM_PAYLOAD_SIZE: usize = 100;
pub const HEADER_SIZE: usize = 8;
pub const MAXIMUM_MESSAGE_SIZE: usize = MAXIMUM_PAYLOAD_SIZE + HEADER_SIZE;
pub const MAGIC_NUMBER_HEADER_OFFSET: usize = 0;
pub const PAYLOAD_SIZE_HEADER_OFFSET: usize = 4;
pub const CODE_HEADER_OFFSET: usize         = 6;
pub const PAYLOAD_HEADER_OFFSET: usize     = 8;


// Request code included in header of incoming message.
pub enum RequestCode {
    Ping = 1,
    GetStats = 2,
    ResetStats = 3,
    Compress = 4,

}

impl RequestCode {
    pub fn convert_u16_to_request_code(value: u16) -> Option<RequestCode> {
        match value {
            1 => Some(RequestCode::Ping),
            2 => Some(RequestCode::GetStats),
            3 => Some(RequestCode::ResetStats),
            4 => Some(RequestCode::Compress),
            _ => None,
        }
    }
}

pub enum ResponseCode {
    Ok = 0,
    UnknownError = 1,
    MessageTooLarge = 2,
    UnsupportedRequestType = 3,
    MagicNumberIncorrect = 34,
    HeaderTooSmall = 35,
    PayloadTooLarge = 36,
    PayloadSizeMismatch = 37,
    PayloadInvaliCases = 38,
    // TODO Add Implementer Defined Response Codes
    // TODO Document these! 
}