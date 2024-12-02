use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut safe_reports = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            continue;
        }

        let levels: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_safe(&levels) || safe_with_dampener(&levels) {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    // Viss listen er mindre enn 2 så funke det ikkje, så derfor false
    if levels.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    let first_diff = diffs[0];
    let first_sign = first_diff.signum();

    if first_diff == 0 {
        return false;
    }

    for &diff in &diffs {
        if diff == 0 {
            return false;
        }
        if diff.signum() != first_sign {
            return false;
        }
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }

    true
}

fn safe_with_dampener(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        if is_safe(&modified_levels) {
            return true;
        }
    }
    false
}
