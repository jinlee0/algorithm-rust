use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    buf.split_ascii_whitespace()
        .flat_map(str::parse::<u32>)
        .for_each(|i| {
            let n = 3_usize.pow(i);
            let mut v = vec!['-'; n];
            recur(&mut v, 0, n);
            writeln!(output, "{}", v.iter().collect::<String>()).unwrap();
        });
    println!("{}", output);
}

fn recur(v: &mut Vec<char>, s: usize, e: usize) {
    if e - s <= 1 {
        return;
    }
    let left = (e - s) / 3 + s;
    let right = (e - s) / 3 * 2 + s;
    v[left..right].fill(' ');
    recur(v, s, left);
    recur(v, right, e);
}