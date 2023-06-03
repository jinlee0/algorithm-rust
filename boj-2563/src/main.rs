use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut canvas = [[false;100];100];
    let mut count = 0;
    buf.lines()
        .skip(1)
        .for_each(|s| {
            let mut xy = s.split_ascii_whitespace().flat_map(str::parse::<usize>);
            let (x, y) = (xy.next().unwrap(), xy.next().unwrap());
            for i in x..x+10 {
                for j in y..y+10 {
                    if !canvas[i][j] {
                        canvas[i][j] = true;
                        count += 1;
                    }
                }
            }
        });
    println!("{}", count);
}
