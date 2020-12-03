use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("input");

    let map: Vec<&str> = input.lines().collect();
    let mapwidth: usize = map[0].trim().chars().count();
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut total: usize = 1;
    for slope in slopes.iter() {
        let mut trees: usize = 0;
        let mut pos: (usize, usize) = (0, 0);
        loop {
            pos = ((pos.0 + slope.0) % mapwidth, pos.1 + slope.1);
            if pos.1 >= map.len() { break; }
            if map[pos.1].chars().nth(pos.0).unwrap() == '#' { trees += 1; }
        }
        total *= trees;
    }
    println!("{}", total);
}
