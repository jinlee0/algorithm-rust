use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut stack = vec![];
    for next in buf.split_ascii_whitespace().skip(1).flat_map(str::parse::<u32>) {
        match next {
            0 => {stack.pop();},
            value => stack.push(value)
        }
    }
    println!("{}", stack.iter().sum::<u32>());
}
