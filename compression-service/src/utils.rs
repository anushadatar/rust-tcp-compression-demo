/// Utility functions associated with the TCP Compresison services messages.
use byteorder::{BigEndian, ReadBytesExt};

use crate::compress;
use crate::definitions;
use crate::message;
use crate::stats;

/// Turn the input bytes into an input message type for further processing.
/// Note that this blidnly populates the struct; validation happens when
/// preparing a response to better facilitate error correction.
#[allow(unused)]
pub fn create_input_message(
    input: &[u8],
    output: &mut message::Message,
    stats_tracker: &mut stats::Tracker,
) {
    // Simply populate a message object with the input values.
    // Validation occurs during response creation.
    // u32 magic number
    let mut magic_number = (&input
        [definitions::MAGIC_NUMBER_HEADER_OFFSET..definitions::PAYLOAD_SIZE_HEADER_OFFSET])
        .read_u32::<BigEndian>()
        .unwrap();
    output.set_magic_number(magic_number);

    // u16 payload size
    let mut payload_size = (&input
        [definitions::PAYLOAD_SIZE_HEADER_OFFSET..definitions::CODE_HEADER_OFFSET])
        .read_u16::<BigEndian>()
        .unwrap();
    output.set_payload_size(payload_size);

    // u16 request code
    let mut request_code = (&input
        [definitions::CODE_HEADER_OFFSET..definitions::PAYLOAD_HEADER_OFFSET])
        .read_u16::<BigEndian>()
        .unwrap();
    output.set_code(request_code);

    // If there is a payload present, set it as a vector.
    if (payload_size > 0) {
        let mut payload = input[definitions::PAYLOAD_HEADER_OFFSET
            ..definitions::PAYLOAD_HEADER_OFFSET + payload_size as usize]
            .to_vec();
        let mut payload = input[8..11].to_vec();
        output.set_payload(&mut payload);
    }
}

/// TODO DOCSTRING
pub fn create_output_message(
    input: &mut message::Message,
    output: &mut message::Message,
    stats_tracker: &mut stats::Tracker,
) {
    // If either fails, it will set the output code() value to an error message.
    validate_magic_number(input, output);
    validate_payload(input, output);

    // Execute on the command assuming the header and payload are as specified.
    if output.code() == definitions::ResponseCode::Ok as u16 {
        match definitions::RequestCode::u16_to_request_code(input.code()).unwrap() {
            definitions::RequestCode::Ping => ping_command(),
            definitions::RequestCode::GetStats => get_stats_command(stats_tracker, output),
            definitions::RequestCode::ResetStats => reset_stats_command(stats_tracker),
            definitions::RequestCode::Compress => compress_command(input, output, stats_tracker),
            _ => unsupported_request(output),
        }
    }
    // Error messages should always have an empty payload.
    if output.code() != definitions::ResponseCode::Ok as u16 {
        output.set_payload_size(0);
        output.set_payload(&mut Vec::new());
    }
}

/// TODO
pub fn ping_command() {
    println!("Processing ping command.");
}

/// TODO
pub fn unsupported_request(output: &mut message::Message) {
    println!("Unsupported request received.");
    output.set_code(definitions::ResponseCode::UnsupportedRequestType as u16);
}

/// TODO DOCSTRING
pub fn get_stats_command(stats_tracker: &mut stats::Tracker, output: &mut message::Message) {
    println!("Get stats command received.");
    let mut payload_buffer = [0 as u8; 4];
    stats_tracker.create_stats_payload(&mut payload_buffer);
    output.set_payload(&mut payload_buffer.to_vec());
}

/// TODO DOCSTRING
pub fn reset_stats_command(stats_tracker: &mut stats::Tracker) {
    println!("Reset stats command received.");
    stats_tracker.reset_stats();
}

/// Compress the input message using a simplified prefix encoding scheme, and
/// then set the output payload value to the compressed buffer. Update the payload
/// size and stats accordingly.
/// TODO ADD PARAMS
pub fn compress_command(
    input: &mut message::Message,
    output: &mut message::Message,
    stats_tracker: &mut stats::Tracker,
) {
    let mut output_message = input.payload();
    let output_size = compress::compress_message(input.payload(), &mut output_message);
    if output_size > 0 {
        println!("{}", output_size);
        output.set_payload(&mut output_message);
        stats_tracker.add_to_bytes_compressed_input(input.payload_size() as usize);
        stats_tracker.add_to_bytes_compressed_output(output_size);
        stats_tracker.update_compression_ratio();
        stats_tracker.add_to_bytes_sent(output_size);
        output.set_payload_size(output_size as u16);
    } else {
        output.set_code(definitions::ResponseCode::CompressionFailed as u16);
    }
}

/// Validate that the header has the appropriate magic number as stored in definitions.
/// If the number is incorrect, set the response code accordingly.
/// input: Message struct containing the message received.
/// output: Message struct containing the message to send out.
pub fn validate_magic_number(input: &mut message::Message, output: &mut message::Message) {
    if input.magic_number() == definitions::MAGIC_NUMBER {
        output.set_code(definitions::ResponseCode::Ok as u16);
    } else {
        output.set_code(definitions::ResponseCode::MagicNumberIncorrect as u16);
    }
    output.set_magic_number(definitions::MAGIC_NUMBER);
}

/// Validate that the header has the appropriate magic number as stored in definitions.
/// If the number is incorrect, set the response code accordingly.
/// input: Message struct containing the message received.
/// output: Message struct containing the message to send out.
pub fn validate_payload(input: &mut message::Message, output: &mut message::Message) {
    if input.payload_size() != input.payload().len() as u16 {
        println!("Failed here");
        output.set_code(definitions::ResponseCode::PayloadSizeMismatch as u16);
    }
    // TODO CHECK ALL LOWERCASE OR RESET AS WELL
    if output.code() == definitions::ResponseCode::Ok as u16 {
        output.set_payload_size(input.payload_size());
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions;
    use crate::message;
    use crate::stats;
    use crate::utils;

    // Test that the ping function works as expected.
    #[test]
    fn test_ping() {
        let received_data = [
            83u8,
            84,
            82,
            89,
            0,
            0,
            0,
            definitions::RequestCode::Ping as u8,
        ];
        let mut send_data = [0u8; 8];
        let mut stats_tracker = stats::Tracker::new();
        let mut received_message = message::Message::new();
        let mut send_message = message::Message::new();
        utils::create_input_message(&received_data, &mut received_message, &mut stats_tracker);
        utils::create_output_message(&mut received_message, &mut send_message, &mut stats_tracker);
        send_message.to_bytes(&mut send_data);
        assert_eq!(&send_data[..8], &[83u8, 84, 82, 89, 0, 0, 0, 0]);
    }

    // Todo
    #[test]
    fn test_compress() {
        let received_data = [
            83u8,
            84,
            82,
            89,
            0,
            3,
            0,
            definitions::RequestCode::Compress as u8,
            88,
            88,
            88,
        ];
        let mut send_data = [0u8; 11];
        let mut stats_tracker = stats::Tracker::new();
        let mut received_message = message::Message::new();
        let mut send_message = message::Message::new();
        utils::create_input_message(&received_data, &mut received_message, &mut stats_tracker);
        utils::create_output_message(&mut received_message, &mut send_message, &mut stats_tracker);
        send_message.to_bytes(&mut send_data);

        // Validate with known values
        assert_eq!(
            &send_data[..11],
            &[
                83u8,
                84,
                82,
                89,
                0,
                3,
                0,
                definitions::RequestCode::Compress as u8,
                51,
                88
            ]
        );
        // Validate update to the stats.
        // TODO
    }
}

// TODO ADD TESTS
