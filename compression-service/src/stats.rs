// TODO Block Comment
use crate::message;

#[derive(Debug, Default)]
#[repr(packed)]
pub struct Tracker {
    bytes_read: u32,
    bytes_sent: u32,
    compression_ratio: u8,
}

impl Tracker {
    pub fn new() -> Tracker {
        Default::default()
    }

    pub fn new_with_params(bytes_read: u32, bytes_sent: u32, compression_ratio: u8) -> Tracker {
        Tracker {
            bytes_read: bytes_read, 
            bytes_sent: bytes_sent,
            compression_ratio: compression_ratio,
        }
    }

    pub fn bytes_read(&self) -> u32 {
        return self.bytes_read
    }

    pub fn bytes_sent(&self) -> u32 {
        return self.bytes_sent
    }

    pub fn ompression_ratio(&self) -> u8 {
        return self.compression_ratio
    }

    pub fn add_to_bytes_read(&mut self, added_value: usize) {
        self.bytes_read += added_value as u32
    }

    pub fn add_to_bytes_sent(&mut self, added_value: usize) {
        self.bytes_sent += added_value as u32
    }

    pub fn set_bytes_read(&mut self, value: usize) {
        self.bytes_read = value as u32
    }

    pub fn set_bytes_sent(&mut self, value: usize) {
        self.bytes_sent = value as u32
    }
    
    pub fn set_compression_ratio(&mut self, value: usize) {
        self.compression_ratio = value as u8
    }
    // TODO 
    pub fn update_compression_ratio(&mut self, bytes_compressed: usize, message_total_size: usize) {
        if message_total_size > 0 && bytes_compressed > 0 {
            let new_ratio = bytes_compressed as f64 / message_total_size as f64;
            let ratio = (1f64 - new_ratio) * 100f64;
            self.compression_ratio = ratio as u8;
        }
    }

    pub fn reset_stats(&mut self) {
        self.set_bytes_read(0);
        self.set_bytes_sent(0);
        self.set_compression_ratio(0);
    }

}

pub fn create_stats_payload(input: &mut message::Message) {
    ()
}