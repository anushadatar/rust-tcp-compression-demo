/// Message object used to manage requests and responses over the server.
use crate::definitions;
use byteorder::{BigEndian, ByteOrder};

/// Message object for both requests and responses. Note that this does
/// not include validation by default; populated message objects can
/// have invalid values.
/// magic_number: u32 header check
/// payload_size: u16 length of payload
/// code: u16 request or response code
/// payload: Vec<u8> containing payload
#[derive(Debug, Default)]
#[repr(C)]
pub struct Message {
    magic_number: u32,
    payload_size: u16,
    code: u16,
    payload: Vec<u8>,
}

impl Message {
    /// Default message constructor with empty fields.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the message's magic number
    /// Returns: u32 magic number
    pub fn magic_number(&self) -> u32 {
        return self.magic_number;
    }

    /// Set the message's magic number
    /// magic_number: u32 magic number to set
    pub fn set_magic_number(&mut self, magic_number: u32) {
        self.magic_number = magic_number;
    }

    /// Get the message's payload size
    /// Returns: u16 payload size
    pub fn payload_size(&self) -> u16 {
        return self.payload_size;
    }

    /// Set the message's payload size
    /// magic_number: u16 payload size to set
    pub fn set_payload_size(&mut self, payload_size: u16) {
        self.payload_size = payload_size;
    }

    /// Get the message's code
    /// Returns: u16 code
    pub fn code(&self) -> u16 {
        return self.code;
    }

    /// Set the message's payload size
    /// magic_number: u16 code to set
    pub fn set_code(&mut self, request_code: u16) {
        self.code = request_code;
    }

    /// Get the message's payload
    /// Returns: Vec<u8> payload
    pub fn payload(&self) -> Vec<u8> {
        return self.payload.to_vec();
    }

    /// Set the message's payload.
    /// magic_number: Vec<u8> payload
    pub fn set_payload(&mut self, payload: &mut Vec<u8>) {
        self.payload = payload.to_vec();
    }

    /// Convert a message to bytes as specified by the TCP protocol.
    /// output: The array to write the message's bytes to.
    /// Returns: The size of the output actually written to.
    pub fn to_bytes(&mut self, output: &mut [u8]) -> usize {
        // TODO Make this more variable, maybe even just use Add the offset values here.self
        BigEndian::write_u32(&mut output[definitions::MAGIC_NUMBER_HEADER_OFFSET..definitions::PAYLOAD_SIZE_HEADER_OFFSET], self.magic_number());
        BigEndian::write_u16(&mut output[definitions::PAYLOAD_SIZE_HEADER_OFFSET..definitions::CODE_HEADER_OFFSET], self.payload_size());
        BigEndian::write_u16(&mut output[definitions::CODE_HEADER_OFFSET..definitions::PAYLOAD_HEADER_OFFSET], self.code());
        if self.payload_size() > 0 {
            let mut current_offset = definitions::PAYLOAD_HEADER_OFFSET + 1 as usize;
            let payload = self.payload();
            for current_value in 0..self.payload_size() - 1 {
                output[current_offset] = payload[current_value as usize];
                current_offset += 1;
            }
        }
        return (8 + self.payload_size()) as usize;
    }
}
