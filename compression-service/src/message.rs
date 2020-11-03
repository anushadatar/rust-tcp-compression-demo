/// TODO Doc comment
use crate::definitions;
use byteorder::{BigEndian, ByteOrder};

/// TODO doc comment
#[derive(Debug, Default)]
#[repr(C)]
pub struct Message {
    magic_number: u32,
    payload_size: u16,
    code: u16,
    payload: Vec<u8>,
}

/// TODO doc comment
impl Message {
    /// TODO doc comment
    pub fn new() -> Self {
        Default::default()
    }

    /// TODO doc comment
    pub fn magic_number(&self) -> u32 {
        return self.magic_number;
    }

    /// TODO doc comment
    pub fn set_magic_number(&mut self, magic_number: u32) {
        self.magic_number = magic_number;
    }

    /// TODO doc comment
    pub fn payload_size(&self) -> u16 {
        return self.payload_size;
    }

    /// TODO doc comment
    pub fn set_payload_size(&mut self, payload_size: u16) {
        self.payload_size = payload_size;
    }

    /// TODO doc comment
    pub fn code(&self) -> u16 {
        return self.code;
    }

    /// TODO doc comment
    pub fn set_code(&mut self, request_code: u16) {
        self.code = request_code;
    }

    /// TODO doc comment
    pub fn payload(&self) -> Vec<u8> {
        return self.payload.to_vec();
    }

    /// TODO doc comment
    pub fn set_payload(&mut self, payload: &mut Vec<u8>) {
        self.payload = payload.to_vec();
    }

    /// TODO doc comment
    pub fn to_bytes(&mut self, output: &mut [u8]) {
        // TODO Make this more variable, maybe even just use Add the offset values here.self
        BigEndian::write_u32(&mut output[0..4], self.magic_number());
        BigEndian::write_u16(&mut output[4..6], self.payload_size());
        BigEndian::write_u16(&mut output[6..], self.code());
        if self.payload_size() > 0 {
            let mut current_offset = definitions::PAYLOAD_HEADER_OFFSET as usize;
            let payload = self.payload();
            for current_value in 0..self.payload_size() - 1 {
                output[current_offset] = payload[current_value as usize];
                current_offset += 1;
            }        
        // TODO payload
        }
    }
}
// TODO ADD TESTS