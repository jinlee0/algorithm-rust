use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);
    let (mut n, b) = (tokens.next().unwrap(), tokens.next().unwrap()); 
    let mut answer = String::new();
    while n > 0 {
        let remainder = n % b;
        n /= b;
        answer.push(char::from_digit(remainder, b).unwrap());
    }
    println!("{}", answer.chars().rev().collect::<String>().to_uppercase());
}
