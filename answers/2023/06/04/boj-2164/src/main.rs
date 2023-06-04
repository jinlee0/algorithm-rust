use std::{io, collections::VecDeque};

fn main() {
    let n = io::read_to_string(io::stdin()).unwrap().trim().parse::<u32>().unwrap();
    let mut q = (1..=n).collect::<VecDeque<_>>();
    while q.len() != 1 {
        q.pop_front();
        q.rotate_left(1);
    }
    println!("{}", q[0]);
}
