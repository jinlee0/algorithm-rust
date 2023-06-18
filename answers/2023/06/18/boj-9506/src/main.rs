use std::io;

fn main() -> io::Result<()> {
    let buf = io::read_to_string(io::stdin())?;
    let answer = buf.lines()
        .take_while(|line| line != &"-1")
        .flat_map(str::parse::<usize>)
        .map(|i| {
            let mut factors = vec![];
            let mut sum = 0;
            for f in 1..i as usize {
                if i % f == 0 {
                    factors.push(f.to_string());
                    sum += f;
                }
            }
            match sum == i {
                true => format!("{i} = {}", factors.join(" + ")),
                false => format!("{i} is NOT perfect.")
            }
        })
    .reduce(|acc, e| format!("{acc}\n{e}"))
        .unwrap();
    println!("{answer}");
    Ok(())
}

