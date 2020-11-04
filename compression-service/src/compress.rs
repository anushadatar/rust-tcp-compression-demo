/// Compress a message associated 

/// Compress a message using a simplified prefix encoding scheme
/// input: Vector<u8> to compress
/// Returns: Compressed vector or None
pub fn compress_message(input: Vec<u8>) -> Option<Vec<u8>> {
    let input_slice = input.as_slice();
    let output_slice = input_slice.clone();
    let mut output_vector = vec![0 as u8; input_slice.len()];
    let input_length = input_slice.len();
    let mut total_count = 1;
    let mut compressed_count = 0;
    if (input_slice.len() < output_slice.len()) || input_length == 0 {
        return None;
    }
    for i in 0..input_length {
        if i == input_length - 1 || input_slice[i] != input_slice[i + 1] {
            if total_count == 2 {
                output_vector[compressed_count] = input_slice[i];
                println!("{}", output_vector[compressed_count]);
                compressed_count += 1;
            }
            if total_count > 2 {
                for character in total_count.to_string().bytes() {
                    output_vector[compressed_count] = character;
                    println!("{}", output_vector[compressed_count]);
                    compressed_count += 1;
                }
            }
            output_vector[compressed_count] = input_slice[i];
            println!("{}", output_vector[compressed_count]);
            compressed_count += 1;
            total_count = 0;
        }
        total_count += 1
    }
    println!("{:?}", output_vector);
    output_vector.truncate(compressed_count);
    println!("{:?}", output_vector);

    return Some(output_vector);
}