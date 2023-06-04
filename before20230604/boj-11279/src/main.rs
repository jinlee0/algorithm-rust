use std::{collections::BinaryHeap, io};
use std::fmt::Write;
fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut heap = BinaryHeap::new();
    for i in buf.split_ascii_whitespace().skip(1).flat_map(str::parse::<i32>) {
        if i > 0 {
            heap.push(i);
        } else {
            let e = match heap.pop() {
                Some(e) => e,
                None => 0
            };
            writeln!(output, "{}", e).unwrap();
        }
    }
    println!("{}", output);
}
