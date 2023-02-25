use std::io::{Read, stdin};
use std::usize;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_ascii_whitespace().map(str::parse).flatten();
    let a: usize = numbers.next().unwrap();
    let b = numbers.next().unwrap();
    println!("{}", a + b);
}
