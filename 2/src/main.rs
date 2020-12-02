use std::io::{self, Read};
use regex::Regex;

fn policy_ok(policy: &str, password: &str) -> bool {
    let re = Regex::new(r"^(\d+)-(\d+) (.)").unwrap();
    let caps = re.captures(policy).unwrap();
    let lo: usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let hi: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let ch: char = caps.get(3).unwrap().as_str().chars().next().unwrap();

    let p1: char = password.chars().nth(lo - 1).unwrap_or('\0');
    let p2: char = password.chars().nth(hi - 1).unwrap_or('\0');

    (ch == p1 && ch != p2) || (ch != p1 && ch == p2)
}

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("input");

    let num_valid: u32 = buffer.lines().map(|line| {
        let v: Vec<&str> = line.split(':').collect();
        return if policy_ok(v[0].trim(), v[1].trim()) { 1 } else { 0 }
    }).sum();
    println!("{}", num_valid);
}
