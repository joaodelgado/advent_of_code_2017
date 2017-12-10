use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    let mut input = String::new();
    {
        let mut f = File::open(path).expect("input file not found");
        String::new();
        f.read_to_string(&mut input).expect(
            "something went wrong while reading the input file",
        );
    }

    input
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
