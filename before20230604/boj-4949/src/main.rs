use std::collections::HashMap;
use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut stack = Vec::<char>::with_capacity(100);
    let map = HashMap::from([(']', '['), (')', '(')]);
    'start: for line in buf.lines() {
        if line == "." { break; }
        stack.clear();
        for c in line.chars() {
            match c {
                '['|'(' => stack.push(c),
                ']'|')' => {
                    if stack.is_empty() || &stack.pop().unwrap() != map.get(&c).unwrap() {
                        writeln!(output, "{}", "no").unwrap();
                        continue 'start;
                    }
                }
                _ => {}
            }
        }
        if stack.is_empty() {
            writeln!(output, "{}", "yes").unwrap();
        } else {
            writeln!(output, "{}", "no").unwrap();
        }
    }
    println!("{}", output);
}

#[test]
fn test() {
    let buf = "[.
).
.";
let mut output = String::new();
let mut stack = Vec::<char>::with_capacity(100);
let map = HashMap::from([(']', '['), (')', '(')]);
'start: for line in buf.lines() {
    if line == "." {
        break;
    }
    stack.clear();
    for c in line.chars() {
        match c {
            '['|'(' => stack.push(c),
            ']'|')' => {
                if stack.is_empty() || &stack.pop().unwrap() != map.get(&c).unwrap() {
                    writeln!(output, "{}", "no").unwrap();
                    continue 'start;
                }
            }
            _ => {}
        }
    }
    if stack.is_empty() {
        writeln!(output, "{}", "yes").unwrap();
    } else {
        writeln!(output, "{}", "no").unwrap();
    }
}
println!("{}", output);
}