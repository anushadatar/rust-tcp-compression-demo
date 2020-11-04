/// Definitions associated with this TCP compression service implementation.

// The magic number present at the beginning of each message.
pub const MAGIC_NUMBER: u32 = 0x53545259;
// The (user-defined) maximum payload size, can be modified as needed.
pub const MAXIMUM_PAYLOAD_SIZE: usize = 100;
// Specified header size.
pub const HEADER_SIZE: usize = 8;
// Maxiumum message size (where message has header and payload)
pub const MAXIMUM_MESSAGE_SIZE: usize = MAXIMUM_PAYLOAD_SIZE + HEADER_SIZE;
// Header offsets that match specified format of [magic number, payload size, code, payload]
pub const MAGIC_NUMBER_HEADER_OFFSET: usize = 0;
pub const PAYLOAD_SIZE_HEADER_OFFSET: usize = 4;
pub const CODE_HEADER_OFFSET: usize = 6;
pub const PAYLOAD_HEADER_OFFSET: usize = 8;

// Request code included in header of incoming message.
pub enum RequestCode {
    Ping = 1,
    GetStats = 2,
    ResetStats = 3,
    Compress = 4,
}
// Convert u16 values from headers into individual request codes.
impl RequestCode {
    pub fn u16_to_request_code(value: u16) -> Option<RequestCode> {
        match value {
            1 => Some(RequestCode::Ping),
            2 => Some(RequestCode::GetStats),
            3 => Some(RequestCode::ResetStats),
            4 => Some(RequestCode::Compress),
            _ => None,
        }
    }
}

// Response code to include in header of message to send out.
pub enum ResponseCode {
    Ok = 0,
    UnknownError = 1,
    MessageTooLarge = 2,
    UnsupportedRequestType = 3,
    // Custom response type documentation is in the README.
    MagicNumberIncorrect = 34,
    PayloadInvalidCases = 35,
    PayloadSizeMismatch = 36,
    CompressionFailed = 37,
}
