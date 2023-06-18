use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let mut tokens = buf.split_ascii_whitespace()
        .flat_map(str::parse::<u32>);
    let mut next = || tokens.next().unwrap();
    let (x, y, w, h) = (next(), next(), next(), next());
    let answer = (x-0).min(y-0).min(x.abs_diff(w)).min(y.abs_diff(h));
    println!("{answer}");
    Ok(())
}
