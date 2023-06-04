use std::io::{self, Write};
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let buf = io::read_to_string(io::stdin())?;
    let mut output = io::BufWriter::new(io::stdout());
    let mut tokens = buf.split_ascii_whitespace();
    let n = tokens.next().unwrap().parse()?;
    let mut dq = std::collections::VecDeque::new();
    for _ in 0..n {
        match tokens.next().unwrap() {
            "push_front" => dq.push_front(tokens.next().unwrap()),
            "push_back" => dq.push_back(tokens.next().unwrap()),
            "pop_front" => writeln!(output, "{}", dq.pop_front().unwrap_or("-1"))?,
            "pop_back" => writeln!(output, "{}", dq.pop_back().unwrap_or("-1"))?,
            "size" => writeln!(output, "{}", dq.len()).unwrap(),
            "empty" => writeln!(output, "{}", u8::from(dq.is_empty()))?,
            "front" => writeln!(output, "{}", dq.front().unwrap_or(&"-1"))?,
            "back" => writeln!(output, "{}", dq.back().unwrap_or(&"-1"))?,
            _ => ()
        }
    }
    Ok(())
}
