use std::{io, collections::HashSet};

fn main() -> io::Result<()> {
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();
    let buf = io::read_to_string(io::stdin())?;
    buf.lines()
        .for_each(|line| {
            let mut tokens = line.split_ascii_whitespace().flat_map(str::parse::<u32>);
            let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
            if xs.contains(&x) {
                xs.remove(&x);
            } else {
                xs.insert(x);
            }
            if ys.contains(&y) {
                ys.remove(&y);
            } else {
                ys.insert(y);
            }
        });
    println!("{} {}", xs.iter().next().unwrap(), ys.iter().next().unwrap());
    Ok(())
}
