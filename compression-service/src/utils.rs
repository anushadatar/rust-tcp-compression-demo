use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use byteorder::{BigEndian, ReadBytesExt};

use crate::compress;
use crate::definitions;
use crate::message;
use crate::stats;

#[allow(unused)]
pub fn create_input_message(input: &[u8], output: &mut message::Message, stats_tracker: &mut stats::Tracker) {
    // Simply populate a message object with the input values.
    // Validation occurs during response creation. 
    // TODO Add error checking to this.
    // TODO this is really error unsafe but let's get moving
    let mut magic_number = (&input[definitions::MAGIC_NUMBER_HEADER_OFFSET..definitions::PAYLOAD_SIZE_HEADER_OFFSET ]).read_u32::<BigEndian>().unwrap();
    let mut payload_size = (&input[definitions::PAYLOAD_SIZE_HEADER_OFFSET..definitions::CODE_HEADER_OFFSET]).read_u16::<BigEndian>().unwrap();
    let mut request_code = (&input[definitions::CODE_HEADER_OFFSET..definitions::PAYLOAD_HEADER_OFFSET]).read_u16::<BigEndian>().unwrap();
    output.set_magic_number(magic_number);
    output.set_payload_size(payload_size);
    output.set_code(request_code);
    
    if (payload_size > 0) {
        let mut payload = input[definitions::PAYLOAD_HEADER_OFFSET..definitions::PAYLOAD_HEADER_OFFSET+payload_size as usize].to_vec();
        output.set_payload(payload);
    }
}

pub fn create_output_message(input: &mut message::Message, output: &mut message::Message, stats_tracker: &mut stats::Tracker) {
    validate_header_and_set_response_code(input, output);
    
    // Validate the message and update the header accordingly. Individual methods
    if input.payload_size() > 0 {
        validate_and_set_payload_length(input, output);
    }

    // Execute on the command assuming the header and payload are as specified.
    if output.code() == 0 {
        match definitions::RequestCode::convert_u16_to_request_code(input.code()).unwrap() {
            definitions::RequestCode::Ping => ping_command(input, output),
            definitions::RequestCode::GetStats => get_stats_command(stats_tracker, output),
            definitions::RequestCode::ResetStats => reset_stats_command(stats_tracker),
            definitions::RequestCode::Compress => compress_command(input, output),
        }
    }
}

pub fn ping_command(input: &mut message::Message, output: &mut message::Message) {
    // TODO is there an error that could pass through here???
    println!("Executing on the ping command")
}
pub fn get_stats_command(stats_tracker: &mut stats::Tracker, output: &mut message::Message) {
    // TODO Create and set a payload here.
    stats::create_stats_payload(output);
}

pub fn reset_stats_command(stats_tracker: &mut stats::Tracker) {
    stats_tracker.reset_stats();
}

pub fn compress_command(input: &mut message::Message, output: &mut message::Message) {
    // TODO Create and set a payload here.
    let output_size = compress::compress_message(input.payload(), output.payload())
    // TODO Error checking and update satats.
} 


pub fn validate_header_and_set_response_code(input: &mut message::Message, output: &mut message::Message) {
    // TODO clean this up! 
    if input.validate_magic_number() != true {
        output.set_code(definitions::ResponseCode::MagicNumberIncorrect as u16);
    }
    if input.validate_request_code() != true {
        output.set_code(definitions::ResponseCode::UnsupportedRequestType as u16); 
    }
    output.set_magic_number(definitions::MAGIC_NUMBER);
    output.set_payload_size(0 as u16);
}

pub fn validate_and_set_payload_length(input: &mut message::Message, output: &mut message::Message) {
    // Check if length matches length
    // Check if all lowercase
}
#[cfg(test)]
mod tests {
    
    use crate::definitions;
    use crate::message;
    use crate::stats;
    use crate::utils;
    
    #[test]
    fn test_ping() {
        let received_data = [83u8, 84, 82, 89, 0, 0, 0, definitions::RequestCode::Ping as u8];
        let mut send_data = [0u8; 8];
        let mut stats_tracker = stats::Tracker::new();
        let mut received_message = message::Message::new();
        let mut send_message = message::Message::new();
        utils::create_input_message(&received_data, &mut received_message, &mut stats_tracker);
        utils::create_output_message(&mut received_message, &mut send_message,  &mut stats_tracker);
        println!("{}", send_message.magic_number());
        send_message.to_bytes(&mut send_data);
        assert_eq!(&send_data[..8], &[83u8, 84, 82, 89, 0, 0, 0, 0]);
    }

}
