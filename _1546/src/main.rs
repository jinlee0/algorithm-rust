use std::io::stdin;

fn read_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

fn main() {
    read_line();
    let values = read_line()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let &max = values.iter().max().unwrap();
    let mut sum = 0.0;
    for &i in values.iter() {
        sum = sum + i as f64 / max as f64 * 100.0;
    }
    let answer = sum / values.len() as f64;
    println!("{}", answer)
}
