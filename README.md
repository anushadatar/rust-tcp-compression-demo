# TCP Compression Service

## Build and Test
### Target Platform
I conducted this program's development on a machine running Ubuntu 20.04 and Rust 2018.
### Setup and Workflow
• A shell script named build.sh that does all the necessary work to setup
and/or build your project
• A shell script named run.sh which starts your executable listening on port
4000
## Architecture
– A brief description of the inner workings of your code
## Design Decisions
### Assumptions
– A description of any assumptions that you made while implementing

## Resources
### Libraries
-  
### Resources
– A list of third party libraries or other projects which you used along
with a very short description of why you used that particular library
- As I'm still fairly new to Rust, I referenced [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/index.html) to gain context and solve problems.

## Future Improvements
If I had additional time and resources to dedicate to this project, I would extend this project in two major directions.

- Error Checking and Testing: The testing present here does not necessarily encapsulate all use cases and potential opportunities for user error. For example, while there are some higher-level unit tests, I have minimal end-to-end integration testing support in place that confirms that the server 
- Functionality: This daemon could support additional components of the DNS specification and more sophisticated user interface features (like supporting loading a configuration file for address redirection).
