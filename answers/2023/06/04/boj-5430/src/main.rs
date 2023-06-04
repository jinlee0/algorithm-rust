use std::{io, collections::VecDeque, str::Chars};
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut lines = buf.lines();
    let k = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..k {
        let ops = lines.next().unwrap().chars();
        let elements = lines.by_ref().skip(1).next().unwrap().trim_matches(|c| c == '[' || c== ']');
        let v = if elements.is_empty() {VecDeque::new()} else { elements.split(',').collect::<VecDeque<_>>()};
        match execute(ops, v) {
            Ok(v) => writeln!(output, "{}", v).unwrap(),
            Err(_) => writeln!(output, "error").unwrap()
        }
    }
    println!("{}", output);
}

fn to_string(v: VecDeque<&str>) -> String {
    let mut inner = v.iter().fold(String::new(), |mut acc, s| {
        write!(acc, "{},", s).unwrap();
        acc
    });
    if !inner.is_empty() {
        inner.remove(inner.len()-1);
    }
    format!("[{}]", inner)
}

fn execute(ops: Chars, mut v: VecDeque<&str>) -> Result<String, ()>{
    let mut asc = true;
    for op in ops {
        match op {
            'R' => asc = !asc,
            'D' => {
                let popped = if asc {v.pop_front()} else {v.pop_back()};
                if popped.is_none() { return Err(())}
            },
            _ => unreachable!()
        }
    }
    if !asc {
        v = v.into_iter().rev().collect();
    }
    Ok(to_string(v))
}

#[test]
fn test() {
    let ops = "D".chars();
    let elements = "[]".trim_matches(|c| c == '[' || c== ']');
    let v = if elements.is_empty() {VecDeque::new()} else { elements.split(',').collect::<VecDeque<_>>()};
    println!("{:?} {:?}", ops, v);
    let mut output = String::new();

    match execute(ops, v) {
        Ok(v) => writeln!(output, "{}", v).unwrap(),
        Err(_) => writeln!(output, "error").unwrap()
    }
    println!("{}", output);
}