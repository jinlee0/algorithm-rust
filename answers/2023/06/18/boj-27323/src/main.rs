use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let mut tokens = buf.split_ascii_whitespace()
        .flat_map(str::parse::<u32>);
    let (a, b) = (tokens.next().unwrap(), tokens.next().unwrap());
    println!("{}", a * b);
    Ok(())
}
