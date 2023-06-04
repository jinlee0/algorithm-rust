use std::{io};
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let (n, k) = (input.next().unwrap(), input.next().unwrap());
    let mut q = (1..=n).collect::<Vec<_>>();
    let mut curr = (k - 1) % q.len();
    let mut buf = q.remove(curr).to_string();
    while !q.is_empty() {
        curr = (curr + k - 1) % q.len();
        buf = buf + ", " + q.remove(curr).to_string().as_str();
    }
    println!("<{}>", buf);
}
