use std::{io::{stdin, Read}, collections::HashSet};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_ascii_whitespace().map(|s| s.to_string());
    input.next();
    let count = input.filter(|s| is_group(s)).count();
    println!("{}", count)
}

fn is_group(s: &String) -> bool{
    let mut chars = s.chars().peekable();
    let mut set= HashSet::new();
    loop {
        if chars.peek().is_none() {
            return true;
        }
        let token = chars.next().unwrap();
        if set.contains(&token) {
            return false
        }
        set.insert(token);
        loop {
            if chars.peek().is_none() {
                return true;
            }
            if chars.peek().unwrap() == &token {
                chars.next();
            } else {
                break;
            }
        }
    }
}
