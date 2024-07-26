# base64
A simple Rust library for Base64 encoding and decoding.

## Installation

Add this to your `Cargo.toml`:
```
[dependencies]
base64 = "0.1.0"
```
Then, run cargo build to download and compile the library.

## Usage
```
extern crate base64;

use base64::{encode, decode};

fn main() {
    let original = "Hello, world!";
    let encoded = encode(original);
    let decoded = decode(&encoded);

    println!("Original: {}", original);
    println!("Encoded: {}", encoded);
    println!("Decoded: {}", decoded);
}
```

## Functions
- **encode**: Encodes a string into Base64 format.
 1. Parameters: `data: &str` : The input string to encode.
 2. Returns: `String` : The Base64 encoded string.

- **encode**: Decodes a Base64 string back into the original string.
 1. Parameters: `data: &str` : The Base64 encoded string to decode.
 2. Returns: `String` : The decoded original string.