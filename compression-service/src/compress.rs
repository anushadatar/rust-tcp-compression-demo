/// Compress a message associated 

/// The  
pub fn compress_message(input: Vec<u8>, output: &mut Vec<u8>) -> usize {
    let input_slice = input.as_slice();
    let output_slice = output.as_mut_slice();

    let input_length = input_slice.len();
    let mut total_count = 1;
    let mut compressed_count = 0;
    if (input_slice.len() < output_slice.len()) || input_length == 0 {
        return 0;
    }

    for i in 0..input_length {
        if i == input_length - 1 || input_slice[i] != input_slice[i + 1] {
            if total_count == 2 {
                output_slice[compressed_count] = input_slice[i];
                compressed_count += 1;
            }
            if total_count > 2 {
                for character in total_count.to_string().bytes() {
                    output_slice[compressed_count] = character;
                    compressed_count += 1;
                }
            }
            output_slice[compressed_count] = input_slice[i];
            compressed_count += 1;
            total_count = 0;
        }
        total_count += 1
    }
    output_slice.to_vec();
    return compressed_count;
}