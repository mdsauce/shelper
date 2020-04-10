# Sauce Help
Rust CLI wrapper and library for the Sauce Labs public API.  Obtain Job details for specific jobs, builds, and Virtual Desktop, iOS Simulator, & Android Emulator sessions.

## Development
`cargo doc --open`: compile and run the documentation

`cargo install`: install all the dependencies

`cargo test`: run all tests

`cargo test -- --nocapture some_test_name`: run a specific test & print all the stuff to stdout/stderr

## Testing
Unit tests are not all 100% local. Real credentials are pulled from the env. variables in some cases.  Real HTTP calls are made against the real API.  Do not run tests endlessly as you could get rate limited with a 429.