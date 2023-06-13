use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let mut tokens = buf.split_ascii_whitespace();
    let (n, b) = (
        tokens.next().unwrap(),
        tokens.next().unwrap().parse::<u32>().unwrap()
        );
    let answer = n.chars()
        .flat_map(|c| c.to_digit(b))
        .reduce(|acc, i| acc * b + i)
        .unwrap();
    println!("{}", answer);
    Ok(())
}
