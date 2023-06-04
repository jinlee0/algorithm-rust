use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (a, b, c, d, e, f) = as_tuple(input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap()));
    for x in -999..=999 {
        for y in -999..=999 {
            if (a*x+b*y==c)&&(d*x+e*y==f) {
                println!("{} {}", x, y);
                return;
            }
        }
    }
}

fn as_tuple(mut input: impl Iterator<Item = i32>) -> (i32, i32, i32, i32, i32, i32) {
    (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap()
    )
}
