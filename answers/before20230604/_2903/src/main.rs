use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<u32>().unwrap();

    let mut dots = 4;
    let mut rows = 2;
    for _ in 0..n {
        let boxes = (rows-1) * (rows-1);
        dots += (rows-1) * rows * 2 + boxes;
        rows += rows - 1;
    }
    println!("{}", dots);
    println!("");
}
