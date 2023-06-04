use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut v = buf.split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<i32>)
        .enumerate()
        .collect::<Vec<_>>();
    v.sort_unstable_by_key(|&(_, value)| value);
    
    let mut answer = vec![0; v.len()];
    let mut count = 0;
    for i in 1..v.len() {
        let (index, num) = v[i];
        if num != v[i - 1].1 {
            count += 1;
        }
        answer[index] = count;
    }
    for i in answer {
        writeln!(output, "{}", i).unwrap();
    }
    println!("{}", output);
}
