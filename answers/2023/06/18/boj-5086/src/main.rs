use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let answer = buf
        .lines()
        .take_while(|line| line != &"0 0")
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace().flat_map(str::parse::<f32>);
            let (a, b) = (tokens.next().unwrap(), tokens.next().unwrap());
            match (b % a == 0.0, a % b == 0.0) {
                (true, false) => "factor".to_owned(),
                (false, true) => "multiple".to_owned(),
                _ => "neither".to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{answer}");
    Ok(()) 
}
