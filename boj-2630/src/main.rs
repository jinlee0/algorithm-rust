use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    
    let v = input
        .lines()
        .skip(1)
        .map(|line| line
                .split_ascii_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>();

    let n = v.len();
    let mut answer = Answer{color0:0,color1:0};
    recur(&v, 0, n, 0, n, &mut answer);
    println!("{}\n{}", answer.color0, answer.color1);
}

struct Answer {
    color0: u32,
    color1: u32
}

fn recur(v: &Vec<Vec<u8>>, rs: usize, re: usize, cs: usize, ce: usize, answer: &mut Answer) {
    if is_pure(v, rs, re, cs, ce) {
        if v[rs][cs] == 0 {
            answer.color0 += 1;
        } else {
            answer.color1 += 1;
        }
        return;
    }
    let rh = (re + rs) / 2;
    let ch = (ce + cs) / 2;
    recur(v, rs, rh, cs, ch, answer);
    recur(v, rh, re, cs, ch, answer);
    recur(v, rs, rh, ch, ce, answer);
    recur(v, rh, re, ch, ce, answer);
}

fn is_pure(v: &Vec<Vec<u8>>, rs: usize, re: usize, cs: usize, ce: usize) -> bool{
    let origin = v[rs][cs];
    v
        .iter()
        .skip(rs)
        .take(re-rs)
        .flat_map(|row| row.iter()
            .skip(cs)
            .take(ce-cs))
        .find(|&&i| i != origin).is_none()
}

#[test]
fn exploration() {
    let mut answer = Answer{color0:0,color1:0};
    recur(&vec!(vec![1, 1], vec![0, 0]), 0, 2, 0, 2, &mut answer);
}