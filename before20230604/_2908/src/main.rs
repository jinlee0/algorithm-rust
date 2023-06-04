use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let answer = input
        .split_ascii_whitespace()
        .map(|s| s.chars().rev().collect::<String>().parse::<i32>().unwrap())
        .max().unwrap();
    println!("{}", answer);
}
