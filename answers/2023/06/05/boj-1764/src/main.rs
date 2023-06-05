use std::{io::{self, Write}, collections::HashSet};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut br = io::BufWriter::new(io::stdout());
    let mut lines = buf.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace()
        .flat_map(str::parse::<usize>);
    let (n, _) = (first_line.next().unwrap(), first_line.next().unwrap());

    let v1: HashSet<&str> = lines.by_ref().take(n).collect();
    let v2: HashSet<&str> = lines.collect();

    let mut inter: Vec<&str> = v1.intersection(&v2).map(|s|*s).collect();
    inter.sort();
    writeln!(br, "{}", inter.len()).unwrap();
    for i in inter {
        writeln!(br, "{}", i).unwrap();
    }
}
