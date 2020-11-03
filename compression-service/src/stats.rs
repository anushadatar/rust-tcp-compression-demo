/// Statistics function associated with keeping track of this TCP server instance.
use byteorder::{BigEndian, ByteOrder};

/// TODO doc comment
#[derive(Debug, Default)]
#[repr(packed)]
pub struct Tracker {
    bytes_read: u32,
    bytes_sent: u32,
    compression_ratio: u8,
    bytes_compressed_input: u32,
    bytes_compressed_output: u32,

}

/// TODO doc comment
impl Tracker {
    /// TODO doc comment
    pub fn new() -> Tracker {
        Default::default()
    }

    /// TODO doc comment
    pub fn new_with_params(bytes_read: u32, bytes_sent: u32, compression_ratio: u8, bytes_compressed_input: u32, bytes_compressed_output: u32) -> Tracker {
        Tracker {
            bytes_read: bytes_read,
            bytes_sent: bytes_sent,
            compression_ratio: compression_ratio,
            bytes_compressed_input: bytes_compressed_input,
            bytes_compressed_output: bytes_compressed_output,
        }
    }

    /// Return the number of bytes read since the last initialization or reset.
    /// R
    pub fn bytes_read(&self) -> u32 {
        return self.bytes_read;
    }

    /// TODO doc comment
    pub fn bytes_sent(&self) -> u32 {
        return self.bytes_sent;
    }

    /// TODO doc comment
    pub fn compression_ratio(&self) -> u8 {
        return self.compression_ratio;
    }
    
    /// TODO doc comment
    pub fn bytes_compressed_input(&self) -> u32 {
        return self.bytes_compressed_input;
    }
    /// TODO doc comment
    pub fn bytes_compressed_output(&self) -> u32 {
        return self.bytes_compressed_output;
    }

    /// TODO doc comment
    pub fn add_to_bytes_read(&mut self, added_value: usize) {
        self.bytes_read += added_value as u32
    }

    /// TODO doc comment
    pub fn add_to_bytes_sent(&mut self, added_value: usize) {
        self.bytes_sent += added_value as u32
    }

        /// TODO doc comment
    pub fn add_to_bytes_compressed_input(&mut self, added_value: usize) {
        self.bytes_compressed_input += added_value as u32
    }

    /// TODO doc comment
    pub fn add_to_bytes_compressed_output(&mut self, added_value: usize) {
        self.bytes_compressed_output += added_value as u32
    }

    /// TODO doc comment
    pub fn set_bytes_read(&mut self, value: usize) {
        self.bytes_read = value as u32
    }

    /// TODO doc comment
    pub fn set_bytes_sent(&mut self, value: usize) {
        self.bytes_sent = value as u32
    }

    /// TODO doc comment
    pub fn set_compression_ratio(&mut self, value: usize) {
        self.compression_ratio = value as u8
    }

    /// TODO doc comment
    pub fn set_bytes_compressed_output(&mut self, value: usize) {
        self.bytes_compressed_output = value as u32;   
    }

    /// TODO doc comment
    pub fn set_bytes_compressed_input(&mut self, value: usize) {
        self.bytes_compressed_input = value as u32;
    }

    /// TODO doc comment
    pub fn update_compression_ratio(&mut self) {
        if self.bytes_compressed_input() > 0 && self.bytes_compressed_output() > 0 {
            // Use f64 datatype to increase accuracy.
            let new_ratio = self.bytes_compressed_output() as f64 / self.bytes_compressed_input() as f64;
            let ratio = (1f64 - new_ratio) * 100f64;
            self.set_compression_ratio((ratio as u8).into())
        }
        // TODO maybe add some error checking here.
    }

    /// TODO doc comment
    pub fn reset_stats(&mut self) {
        self.set_bytes_read(0);
        self.set_bytes_sent(0);
        self.set_compression_ratio(0);
    }

    // TODO Add doc comment
    pub fn create_stats_payload(&mut self, payload: &mut [u8]) {
        BigEndian::write_u32(&mut payload[0..4], self.bytes_read());
        BigEndian::write_u32(&mut payload[4..8], self.bytes_sent());
        payload[8] = self.compression_ratio();
    }
}