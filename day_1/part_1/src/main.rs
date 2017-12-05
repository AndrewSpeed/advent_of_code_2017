extern crate captcha;

use std::fs::File;
use std::io::prelude::*;
use captcha::solve_captcha;

pub fn main() {
    let mut file = File::open("./input.txt").expect("file not found");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file.");

    println!("Contents: {}", contents);

    let solution = solve_captcha(String::from(contents.trim()));

    println!("Solution: {}", solution);
}
