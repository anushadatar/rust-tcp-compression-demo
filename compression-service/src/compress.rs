


pub fn compress_message(input:Vec<u8> , output:&mut Vec<u8>) -> usize {
    // TODO make this less obfuscated by at least a little
    println!("running compress data");
    let length_of_message = input.len();
    let mut count = 1;
    let mut compress = 0;
    if (input.len() > output.len()) || length_of_message == 0 {
        return 0;
    }

    for i in 0..length_of_message {
        if i == length_of_message - 1 || input[i] != input[i+1] {
            if count == 2 {
                output[compress] = input[i];
                compress += 1;
            }
            if count > 2 {
                for c in count.to_string().bytes() {
                    output[compress] = c;
                    compress += 1
                }
            }
            output[compress] = input[i];
            compress += 1;
            count = 0;
        }
        count += 1
    }
    // TODO Get the compressed slice in the output.
    return compress

