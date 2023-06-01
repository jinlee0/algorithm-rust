use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut v = Vec::with_capacity(n+1);
    v.push(0);
    let mut sum = 0;
    for _ in 0..n {
        sum += input.next().unwrap();
        v.push(sum);
    }
    let mut output = String::new();
    for _ in 0..m {
        let (i, j)  = (input.next().unwrap(), input.next().unwrap());
        writeln!(output, "{}", v[j]-v[i-1]).unwrap();
    }
    println!("{}", output);
}
