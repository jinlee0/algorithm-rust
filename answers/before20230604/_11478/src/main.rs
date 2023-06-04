use std::{io::stdin, collections::HashSet};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut set = HashSet::new();
    for i in 0..input.len() {
        for j in i+1..=input.len(){
            set.insert(&input[i..j]);
        }
    }
    println!("{}",set.len());
}
