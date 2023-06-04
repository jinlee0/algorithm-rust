use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let (a1, a0, c, n0) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap()
    );
    print!("{}", u8::from(valid(a1, n0, a0, c)));

}

fn valid(a1: i32, n: i32, a0: i32, c: i32) -> bool {
    (a1*n+a0) <= c * n && a1 <= c
}
