use std::io::{self, Write};

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let mut output = io::BufWriter::new(io::stdout());
    let v = buf.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for j in 0..v.iter().map(|l|l.len()).max().unwrap() {
        for i in 0..v.len() {
            v.get(i).unwrap().get(j).and_then(|c| {
                write!(output, "{}", c).unwrap(); 
                Some(c)
            });
        }
    }
    Ok(()) 
}
