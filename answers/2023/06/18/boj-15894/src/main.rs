use std::io;

fn main() -> io::Result<()> {
    let n = io::read_to_string(io::stdin())?.trim().parse::<usize>().unwrap();
    println!("{}", 4*n);
    Ok(())
}
