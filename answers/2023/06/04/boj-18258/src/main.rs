use std::{io, collections::VecDeque};
use std::fmt::Write;
fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut q = VecDeque::new();
    buf.lines()
        .skip(1)
        .map(str::split_ascii_whitespace)
        .for_each(|mut s| {
            match s.next().unwrap() {
                "push" => q.push_back(s.next().unwrap()),
                "pop" => writeln!(output, "{}", q.pop_front().or(Some(&"-1")).unwrap()).unwrap(),
                "size" => writeln!(output, "{}", q.len()).unwrap(),
                "empty" => writeln!(output, "{}", u8::from(q.is_empty())).unwrap(),
                "front" => writeln!(output, "{}", q.front().or(Some(&"-1")).unwrap()).unwrap(),
                "back" => writeln!(output, "{}", q.back().or(Some(&"-1")).unwrap()).unwrap(),
                _ => unreachable!()
            }
        });
    println!("{}", output);
}
