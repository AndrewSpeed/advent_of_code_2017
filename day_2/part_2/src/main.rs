extern crate checksum;

use std::fs::File;
use std::io::prelude::*;
use checksum::sum_divisible_values;

pub fn main() {
    let mut file = File::open("./input.txt").expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Something went wrong reading the file.",
    );

    println!("Contents: {}", contents);

    let solution = sum_divisible_values(contents.trim());

    println!("Solution: {}", solution);
}
