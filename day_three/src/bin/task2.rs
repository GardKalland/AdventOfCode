use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(
        r"(?P<mul>mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))",
    )
    .unwrap();

    let mut total: u64 = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        } else if enabled {
            let x: u64 = cap.name("x").unwrap().as_str().parse().unwrap();
            let y: u64 = cap.name("y").unwrap().as_str().parse().unwrap();
            total += x * y;
        }
    }

    println!("{}", total);
}
