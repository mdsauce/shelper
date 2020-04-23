# Sauce Help
Rust CLI wrapper and library for the Sauce Labs public API.  Obtain Job details for specific jobs, builds.  A Job right now is: Virtual Desktop, iOS Simulator, & Android Emulator tests.

## Installation
Maybe use cargo?  Unsure

Right now best thing is:
1. Git clone
2. `cargo build --release`
3. Find the .exe in `./target/release/shelper`

## Usage
`shelper -h` or `shelper --help`

`shelper -j some_session_id -j some_session_id` or `shelper -j some_session_id some_session_id some_session_id` are both valid.

## Development
`cargo doc --open`: compile and run the documentation

`cargo install`: install all the dependencies

`cargo test`: run all tests

`cargo test -- --nocapture some_test_name`: run a specific test & print all the stuff to stdout/stderr

## Testing
Unit tests are not all 100% local. Real credentials are pulled from the env. variables in some cases.  Real HTTP calls are made against the real API.  Do not run tests endlessly as you could get rate limited with a 429.