use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("input");

    let v: Vec<u64> = buffer.lines().map(|line| line.trim().parse().expect("not a number")).collect();

    for (pa, a) in v.iter().enumerate() {
        for (pb, b) in v.iter().skip(pa + 1).enumerate() {
            for c in v.iter().skip(pb + 1) {
                if a + b + c == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    return;
                }
            }
        }
    }
}
