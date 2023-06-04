use std::{io, collections::HashSet};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sets: Vec<HashSet<i32>> = buf.lines()
        .skip(1)
        .map(|s| s.split_ascii_whitespace().flat_map(str::parse::<i32>).collect::<HashSet<_>>())
        .collect();
    let a = sets[0].len();
    let b = sets[1].len();
    let inter = sets[0].intersection(&sets[1]).count();
    println!("{}", a + b - inter * 2);
}
