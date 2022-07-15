# Example of How to make an HTTP Request in Rust
This is an example of how to make an HTTP Request in Rust using [Reqwest](https://crates.io/crates/reqwest), [Tokio](https://crates.io/crates/tokio), to fetch data from the Cat Fact Ninja API. 
More specifically, the code makes a request to the https://catfact.ninja/fact GET endpoint to randomly fetch a cat fact.
The code shows how to deserialize the response body as a JSON by leveraging [Serde](https://crates.io/crates/serde) `Deserialize` macro.

If you need a thorough explanation, check out the article https://www.becomebetterprogrammer.com/rust-how-to-make-an-http-request.

## Requirements
This example codebase uses Rust programming langauge. Hence, you must have Rust properly installed in your machine if you want to run the code.

Refer to https://www.rust-lang.org/tools/install to setup Rust in your local machine.

## Do you want to run the code?
Clone this repo to your local machine. Open the terminal and run the command `cargo run` command.

