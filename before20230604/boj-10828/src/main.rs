use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut stack: Vec<i32> = vec![];
    for mut input in buf.lines().skip(1).map(str::split_ascii_whitespace) {
        match  input.next().unwrap() {
            "push" => stack.push(input.next().unwrap().parse().unwrap()),
            "pop" =>  writeln!(output, "{}", stack.pop().or(Some(-1)).unwrap()).unwrap(),
            "size" => writeln!(output, "{}", stack.len()).unwrap(),
            "empty" => writeln!(output, "{}", u8::from(stack.is_empty())).unwrap(),
            "top" => writeln!(output, "{}", stack.last().or(Some(&-1)).unwrap()).unwrap(),
            _ => {}
        }
    }    
    println!("{}", output);
}
