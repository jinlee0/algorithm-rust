use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let mut tokens = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let (n, m) = (tokens.next().unwrap(), tokens.next().unwrap());
    let v = tokens.collect::<Vec<_>>()
        .chunks((n*m) as usize)
        .fold(vec![0; (n*m) as usize], |mut acc, v| {
            for i in 0..acc.len() {
                acc[i] += v[i];
            }
            acc
        })
        .chunks(m as usize)
        .map(|v| v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{v}");
    Ok(())
}
