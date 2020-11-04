/// Statistics structure and methods for TCP compression service.

/// Tracker object that manages server metrics and fields.
/// bytes_read : u32 value of total number of bytes read (including headers).
/// bytes_sent : u32 value of total number of bytes sent (including header).
/// compression_ratio: u8 percentage by which bytes have been compressed, as a
/// percentage of total bytes received from the compression algorithm over
/// total bytes sent to compression algorithm.
/// bytes_compressed_input: u32 vale of total number of bytes sent to compression
/// algorithm (based only on payload values).
/// bytes_compressed: u32 value of total number of bytes received from compression
/// algorithm (based only on payload values).
#[derive(Debug, Default)]
pub struct Tracker {
    bytes_read: u32,
    bytes_sent: u32,
    compression_ratio: u8,
    bytes_compressed_input: u32,
    bytes_compressed_output: u32,
}

impl Tracker {
    /// Default message constructor with empty fields.
    pub fn new() -> Tracker {
        Default::default()
    }

    /// Get the trackers's bytes read.
    /// Returns: u32 bytes read.
    pub fn bytes_read(&self) -> u32 {
        return self.bytes_read;
    }

    /// Get the trackers's bytes sent.
    /// Returns: u32 bytes sent.
    pub fn bytes_sent(&self) -> u32 {
        return self.bytes_sent;
    }

    /// Get the trackers's compression ratio.
    /// Returns: u8 compression ratio.
    pub fn compression_ratio(&self) -> u8 {
        return self.compression_ratio;
    }

    /// Get the trackers's bytes compressed input (how many bytes it has tried to compress).
    /// Returns: u32 bytes compressed input.
    pub fn bytes_compressed_input(&self) -> u32 {
        return self.bytes_compressed_input;
    }

    /// Get the trackers's bytes compressed output (how many bytes it has compressed data to).
    /// Returns: u32 bytes compressed output.
    pub fn bytes_compressed_output(&self) -> u32 {
        return self.bytes_compressed_output;
    }

    /// Add to bytes read field.
    /// added_value : u32 value to add to bytes read field.
    /// Returns: Void, modifies bytes read field in place.
    pub fn add_to_bytes_read(&mut self, added_value: u32) {
        self.bytes_read += added_value;
    }

    /// Add to bytes sent field.
    /// added_value : u32 value to add to bytes sent field.
    /// Returns: Void, modifies bytes sent field in place.
    pub fn add_to_bytes_sent(&mut self, added_value: u32) {
        self.bytes_sent += added_value;
    }

    /// Add to byte compressed input field.
    /// added_value : u32 value to add to bytes compressed input field.
    /// Returns: Void, modifies bytes compressed input field in place.
    pub fn add_to_bytes_compressed_input(&mut self, added_value: u32) {
        self.bytes_compressed_input += added_value;
    }

    /// Add to byte compressed output field.
    /// added_value : u32 value to add to bytes compressed output field.
    /// Returns: Void, modifies bytes compressed output field in place.
    pub fn add_to_bytes_compressed_output(&mut self, added_value: u32) {
        self.bytes_compressed_output += added_value;
    }

    /// Set bytes read field.
    /// value : u32 value to set to bytes read field
    /// Returns: Void, modifies bytes read field in place.
    pub fn set_bytes_read(&mut self, value: u32) {
        self.bytes_read = value;
    }

    /// Set bytes sent field.
    /// value : u32 value to set to bytes sent field
    /// Returns: Void, modifies bytes sent field in place.
    pub fn set_bytes_sent(&mut self, value: u32) {
        self.bytes_sent = value;
    }

    /// Set compression ratio field.
    /// value : u8 value to set to compression ratio field
    /// Returns: Void, modifies compression ratio field in place.
    pub fn set_compression_ratio(&mut self, value: u8) {
        self.compression_ratio = value;
    }

    /// Set bytes compressed output field.
    /// value : u32 value to set to bytes compressed output field
    /// Returns: Void, modifies bytes compressed output field in place.
    pub fn set_bytes_compressed_output(&mut self, value: u32) {
        self.bytes_compressed_output = value;
    }

    /// Set bytes compressed input field.
    /// value : u32 value to set to bytes compressed input field
    /// Returns: Void, modifies bytes compressed input field in place.
    pub fn set_bytes_compressed_input(&mut self, value: u32) {
        self.bytes_compressed_input = value;
    }

    /// Update the compression ratio. Call this after updating the bytes for
    /// compressed input and output.
    /// Returns: Void, modifies compression ratio in place.
    pub fn update_compression_ratio(&mut self) {
        if self.bytes_compressed_input() > 0 && self.bytes_compressed_output() > 0 {
            // Use f64 datatype to increase accuracy.
            let new_ratio =
                self.bytes_compressed_output() as f64 / self.bytes_compressed_input() as f64;
            let ratio = (1f64 - new_ratio) * 100f64;
            self.set_compression_ratio((ratio as u8).into())
        } else {
            self.set_compression_ratio(0 as u8);
        }
    }

    /// Reset stats associated with the TCP server instance.
    /// Returns: Void, modifies tracker object in place.
    pub fn reset_stats(&mut self) {
        self.set_bytes_read(0);
        self.set_bytes_sent(0);
        self.set_compression_ratio(0);
        self.set_bytes_compressed_input(0);
        self.set_bytes_compressed_output(0);
    }
}
