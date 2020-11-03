use crate::definitions;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Default)]
#[repr(C)]
pub struct Message {
    magic_number: u32,
    payload_size: u16,
    code: u16,
    payload: Vec<u8>,
}

impl Message {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn magic_number(&self) -> u32 {
        return self.magic_number;
    }

    pub fn set_magic_number(&mut self, magic_number: u32) {
        self.magic_number = magic_number;
    }

    pub fn payload_size(&self) -> u16 {
        return self.payload_size;
    }

    pub fn set_payload_size(&mut self, payload_size: u16) {
        self.payload_size = payload_size;
    }

    pub fn code(&self) -> u16 {
        return self.code;
    }

    pub fn set_code(&mut self, request_code: u16) {
        self.code = request_code;
    }

    pub fn payload(&self) ->  Vec<u8> {
        return self.payload.to_vec();
    }

    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload.to_vec();
    }

    pub fn validate_magic_number(&mut self) -> bool {
        // Validate that the first two bits equal magic number
        // TODO test
        // TODO Add this validation funciton
        if self.magic_number() == definitions::MAGIC_NUMBER {
            true
        }
        else {
            false
        }
    }

    pub fn validate_request_code(&mut self) -> bool {
        // Validate that the first two bits equal magic number
        // TODO test
        // TODO Add this validation funciton
        true
    }

    pub fn validate_payload_length(&mut self) -> bool {
        // Validate that the first two bits equal magic number
        // TODO test
        // TODO Add this validation funciton
        true
    }

    pub fn to_bytes(&mut self, output: &mut [u8]) {
        // TODO Make this more variable, maybe even just use offsets.
        BigEndian::write_u32(&mut output[0..4], self.magic_number());
        BigEndian::write_u16(&mut output[4..6], self.payload_size());
        BigEndian::write_u16(&mut output[6..], self.code());
        // TODO payload    
    }

}


