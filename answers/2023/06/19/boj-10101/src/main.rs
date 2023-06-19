use std::{io, collections::HashSet};

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let v = buf.split_ascii_whitespace()
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();
    let s = v.iter().collect::<HashSet<_>>();
    let answer = match (is_all_60(&v), v.iter().sum::<u32>() == 180, s.len() == 2, s.len() == 3) {
        (true, _, _, _) => "Equilateral",
        (_, true, true, _) => "Isosceles",
        (_, true, _, true) => "Scalene",
        (_, false, _, _) => "Error",
        _ => unreachable!()
    };
    println!("{answer}");
    Ok(())
}

fn is_all_60(v: &Vec<u32>) -> bool {
    v[0] == 60 && v[1] == 60 && v[2] == 60
}
