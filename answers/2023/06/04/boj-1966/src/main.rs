use std::{io};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut inputs = buf.split_ascii_whitespace()
        .flat_map(str::parse::<usize>);
    let c = inputs.next().unwrap();
    for _ in 0..c {
        let (n, mut m) = (inputs.next().unwrap(), inputs.next().unwrap());
        let mut q = inputs.by_ref().take(n).collect::<Vec<_>>();
        let mut priorities = q.clone();
        priorities.sort_by(|a, b| b.cmp(a));
        let mut count = 1;
        loop {
            if q[0] == priorities[0]  {
                if m == 0 {
                    break;
                } else {
                    m -= 1;
                    q.remove(0);
                    priorities.remove(0);
                    count += 1;
                }
            } else {
                q.rotate_left(1);
                m = *m.checked_sub(1).get_or_insert(q.len()-1);
            }
        }
        println!("{}", count);
    }
}
