use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let (max, min) = (10000, -10000);
    let (mut xs, mut xe, mut ys, mut ye) = (max, min, max, min);

    for _ in 0..n {
        let (x, y) = (input.next().unwrap(), input.next().unwrap());
        xs = xs.min(x);
        xe = xe.max(x);
        ys = ys.min(y);
        ye = ye.max(y);
    }
    println!("{}", (xe-xs) * (ye-ys));
}
