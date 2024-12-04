use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };
    let mut count = 0;

    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            if grid[y as usize][x as usize] == 'A' {
                let diag_chars = [
                    grid[(y - 1) as usize][(x - 1) as usize],
                    grid[(y - 1) as usize][(x + 1) as usize],
                    grid[(y + 1) as usize][(x - 1) as usize],
                    grid[(y + 1) as usize][(x + 1) as usize],
                ];

                let patterns = [
                    ['M', 'S', 'M', 'S'],
                    ['S', 'M', 'S', 'M'],
                    ['M', 'M', 'S', 'S'],
                    ['S', 'S', 'M', 'M'],
                ];

                for pattern in &patterns {
                    if diag_chars[0] == pattern[0]
                        && diag_chars[1] == pattern[1]
                        && diag_chars[2] == pattern[2]
                        && diag_chars[3] == pattern[3]
                    {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
