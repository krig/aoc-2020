use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("input");

    let v: Vec<i32> = buffer.lines().map(|line| line.trim().parse().expect("not a number")).collect();

    for (pa, a) in v.iter().enumerate() {
        for b in v.iter().skip(pa + 1) {
            if a + b == 2020 {
                println!("{} * {} = {}", a, b, a * b);
                return;
            }
        }
    }
}
