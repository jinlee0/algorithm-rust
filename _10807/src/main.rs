use std::io::stdin;

fn read_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf);
    buf
}

fn main() {
    read_line();
    let arr: Vec<i32> = read_line().split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let v = read_line().trim().parse::<i32>().unwrap();
    let answer = arr.into_iter().filter(|&i| i==v).count();
    println!("{}", answer);
}
