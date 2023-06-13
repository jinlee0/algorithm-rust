use std::io;

fn main() -> io::Result<()> {
    let coins = vec![25, 10, 5, 1];
    let answer = io::read_to_string(io::stdin())?.split_ascii_whitespace()
        .skip(1)
        .map(|s| {
            let mut m = s.parse::<u32>().unwrap();
            coins.to_vec()
                .iter()
                .map(|c| {
                    let v = m / c;
                    m %= c;
                    v.to_string()
                })
            .collect::<Vec<String>>()
                .join(" ") 
        })
    .collect::<Vec<_>>()
        .join("\n");
    println!("{answer}");
    Ok(())
}
