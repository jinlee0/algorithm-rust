use std::collections::HashSet;
use std::io::{stdin, self};
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let nums = input.by_ref().take(n as usize).collect::<HashSet<_>>();

    input.skip(1)
        .for_each(|i| writeln!(output, "{}", u8::from(nums.contains(&i))).unwrap());
    println!("{}", output);
}
