# Sauce Help

[![crates.io](https://img.shields.io/crates/v/shelper.svg)](https://crates.io/crates/shelper)
[![Documentation](https://docs.rs/shelper/badge.svg)](https://docs.rs/shelper)

CLI wrapper and library for the Sauce Labs public API.  Obtain Job details for specific jobs, builds.  At the time of this writing a Job is: Virtual Desktop, iOS Simulator, or Android Emulator session. Real Device jobs coming in 2020.

## Installation
`cargo install shelper`

This is the easiest installation method but you will need `cargo` installed. 

#### Installing Cargo for package management
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installs the binary from [crates.io](https://crates.io/crates/shelper).  To install cargo, [copy-paste](https://orly-appstore.herokuapp.com/generate?title=Blindly%20copy-pasting%20shell%20scripts&top_text=what%20could%20go%20wrong%3F&author=sudo%20!!&image_code=16&theme=2&guide_text=The%20Whoops%20Edition&guide_text_placement=bottom_right):

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

Read more here: https://www.rust-lang.org/tools/install

#### Install w/ Cargo

```cargo install shelper```

#### Compile from Source
To install and compile from source (or to develop):
1. Git clone
2. `cargo build --release`
3. Find the .exe in `./target/release/shelper`

## Usage
`shelper -h` or `shelper --help`

#### Examples
`shelper -j 129571b50 -j 15b150b5` 

`shelper -j 195b150b5 159nb15915 1595n159`

`shelper -j https://app.saucelabs.com/tests/123456`

## Development
`cargo doc --open`: compile and run the documentation

`cargo install`: install all the dependencies

`cargo test`: run all tests

`cargo test -- --nocapture some_test_name`: run a specific test & print all the stuff to stdout/stderr

## Testing
Unit tests are not all 100% local. Real credentials are pulled from the env. variables in some cases.  Real HTTP calls are made against the real API.  Do not run tests endlessly as you could get rate limited with a 429.