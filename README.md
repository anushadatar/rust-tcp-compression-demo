# TCP Compression Service
## Build and Test
### Target Platform
I conducted this program's development on a machine running Ubuntu 20.04 and Rust 2018.
All of the libraries used in this program are available via cargo, so the only installation
requirement is [Rust](https://www.rust-lang.org/tools/install). 
### Setup and Workflow
By default, this program runs on port 4000. 
Build with the script `build.sh` and run with the script `run.sh`. Run unit tests by navigating into the
`compression_service` directory and calling `cargo test`. 
## Architecture
The `main.rs` script manages the incoming TCP stream by both listening for and sending messages and
leverages using functions from within `utils.rs`. These functions create `Message` structs that contain
header and payload information, and they execute on specified commands from there and populates a new
`Message` struct in converts back down to bytes to respond with.
### Design Decisions
A major design decision I made here was to use `Message` structs rather than directly modifying the input
buffer associated with the data received over the stream. The reason that I made this call was because I
wanted to prioritize readability and clarity of the program, even if it made the program less efficient. That
being said, it would be perfectly reasonable to write a set of functions that directly modify the input
buffer instead and provide a similar API-level user experience. One way I could have made this easier would
have been to investigate how to automatically serialize the structure into bytes rather than by using a
`to_bytes` function and manually handling this functionality.
### Assumptions
This implementation relies on many assumptions about the user and the workflow they employ.
- The input message is somewhat well-formed, or if it is malformed it is
because the message is completely invalid. For example, this implementation
is not currently robust to if an input message has been shifted over by a bit.
- The user knows about the constraints associated with this system; namely that
it has a specific magic number, header format, and payload format. While the system provides somewhat
descriptive error information if the user does not know this is the case, this would limit their
ability to meaningfully interact with the service.

### Custom Error Codes
I defined the following custom error codes:
| **Name**             	| **u16 Equivalent** 	| **Explanation**                                                      	|
|----------------------	|--------------------	|----------------------------------------------------------------------	|
| MagicNumberIncorrect 	| 34                 	| Follow entire Wall                                                   	|
| PayloadInvalidCases  	| 35                 	| The payload contains values that are not lowercase ASCII characters. 	|
| PayloadSizeMismatch  	| 36                 	| Payload size does not match size given in header.                    	|
| CompressionFailed    	| 37                 	| Message compression failed to execute.                               	|

## Resources
### Libraries
The only third-party library I used was [byteorder](https://docs.rs/byteorder/1.0.0/byteorder/index.html)
to simplify encoding and decoding values in network byte order. The reason that I chose to use this library
was because writing functions to handle encoding and decoding byte values is a straightforward but error-prone
task, so I figured I ought to abstract it away to an external library. This library should be avaialble via cargo
and should not require any additional installation to run and use.
### Tutorials
As I'm still fairly new to Rust (but discussed that this would be the best language for me to use here),
I referenced a few tutorials fairly extensively, especially when I was debugging or trying to make my code
as idiomatic as possible:
- [The Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [The Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
Note that if I made specific searches for implementation-level work, I've provided links to those references inline within the source code.

## Future Improvements
If I had additional time and resources to dedicate to this project, I would extend this project in two major directions.

- **Error Checking and Testing**: The testing present here does not necessarily encapsulate all use cases and potential opportunities
for user error. For example, while there are some higher-level unit tests, I have no real end-to-end integration testing support
in place that confirms that the server and client behave as expected. I also could add lower-level tests; for example, I could
add unit tests for the compression algorithm itself to validate it against edge cases. I could also do more input 
- **Functionality**: There are ways that I could further optimize most of the code her. For example, I could make the compression algorithm
more efficient by directly modifying the vectors rather than using input slices, I could directly modify the message buffer
rather than using a struct, and I could more closely evaluate my use of memory here.
