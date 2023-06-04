use std::{io, vec};
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse::<i32>().unwrap();
    let mut perm = lines.flat_map(str::parse::<i32>).peekable();
    let mut stack = vec![];

    for i in 1..=n {
        stack.push(i);
        writeln!(output, "+").unwrap();
        while perm.peek().is_some() && perm.peek() == stack.last() {
            perm.next();
            stack.pop();
            writeln!(output, "-").unwrap();
        }
    }

    if !stack.is_empty() {
        output = "NO".to_string();
    }
    println!("{}", output);
}
