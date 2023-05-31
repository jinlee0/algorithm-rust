use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    println!("{}", u8::from(s.trim() == s.trim().chars().rev().collect::<String>()));
}
