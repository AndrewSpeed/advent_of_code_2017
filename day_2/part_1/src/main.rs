extern crate checksum;

use std::fs::File;
use std::io::prelude::*;
use checksum::calculate_checksum;

pub fn main() {
    let mut file = File::open("./input.txt").expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Something went wrong reading the file.",
    );

    println!("Contents: {}", contents);

    let solution = calculate_checksum(contents.trim());

    println!("Solution: {}", solution);
}
