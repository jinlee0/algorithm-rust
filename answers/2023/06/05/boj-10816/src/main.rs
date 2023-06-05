use std::{io::{self, Write}, collections::HashMap};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::read_to_string(io::stdin())?;
    let mut output = io::BufWriter::new(io::stdout());
    let mut tokens = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let n = tokens.next().unwrap();
    let mut map = HashMap::new();
    tokens.by_ref().take(n as usize).for_each(|key| {
        map.entry(key).and_modify(|v| *v+=1).or_insert(1);
    });
    writeln!(output, "{}", tokens.skip(1).map(|t| map.get(&t).unwrap_or(&0).to_string()).collect::<Vec<_>>().join(" "))?;
    Ok(())
}
