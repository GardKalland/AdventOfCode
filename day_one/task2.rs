use std::collections::HashMap;
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

        let left_num: i64 = tokens[0].parse().unwrap();
        let right_num: i64 = tokens[1].parse().unwrap();
        left.push(left_num);
        right.push(right_num);
    }
    // veit ikkje koffor eg gidd å bruke hashmaps, men inputs er ikkje stor nokk til at det har noe betydning
    let mut right_counts = HashMap::new();
    for num in right {
        *right_counts.entry(num).or_insert(0u64) += 1;
    }

    let mut total_score: i128 = 0;
    for num in left {
        let count = *right_counts.get(&num).unwrap_or(&0u64);
        total_score += num as i128 * count as i128;
    }

    println!("{}", total_score);
}
