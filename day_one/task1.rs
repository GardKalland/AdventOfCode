use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 2 {
            continue;
        }

        let left_num: i64 = tokens[0].parse().unwrap();
        let right_num: i64 = tokens[1].parse().unwrap();
        left.push(left_num);
        right.push(right_num);
    }

    left.sort_unstable();
    right.sort_unstable();

    let tot_dist: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum();

    println!("{}", tot_dist);
}
