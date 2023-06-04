use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.split_ascii_whitespace();
    let chars = lines.next().unwrap().chars();
    let bomb = lines.next().unwrap().chars().collect::<Vec<_>>();

    let mut v = vec![];
    for c in chars {
        v.push(c);
        if v.ends_with(&bomb) {
            v.truncate(v.len() - bomb.len());
        }
    }
    let answer = {
        if v.is_empty() {
            String::from("FRULA")
        } else {
            v.iter().collect::<String>()
        }
    };
    println!("{}", answer);
}

#[test]
fn test() {
    assert_eq!(vec![1], vec![1]);
}
