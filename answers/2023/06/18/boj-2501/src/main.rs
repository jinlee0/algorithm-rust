use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let mut tokens = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let (n, k) = (tokens.next().unwrap(), tokens.next().unwrap());
    let answer = (1..=n).filter(|i| n % i == 0)
        .nth(k-1)
        .unwrap_or(0);
    println!("{answer}");
    Ok(())
}
