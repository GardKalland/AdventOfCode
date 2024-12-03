use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total: u64 = 0;

    for cap in re.captures_iter(&input) {
        let x: u64 = cap[1].parse().unwrap();
        let y: u64 = cap[2].parse().unwrap();

        total += x * y;
    }

    println!("{}", total);
}
