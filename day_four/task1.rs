use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let word = "XMAS";
    let word_len = word.len();

    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != 'X' {
                continue;
            }
            for &(dx, dy) in &directions {
                let end_x = x as isize + (word_len as isize - 1) * dx;
                let end_y = y as isize + (word_len as isize - 1) * dy;
                if end_x < 0 || end_x >= width as isize || end_y < 0 || end_y >= height as isize {
                    continue;
                }

                let mut match_word = true;
                for k in 1..word_len {
                    let nx = x as isize + k as isize * dx;
                    let ny = y as isize + k as isize * dy;
                    if grid[ny as usize][nx as usize] != word.chars().nth(k).unwrap() {
                        match_word = false;
                        break;
                    }
                }
                if match_word {
                    count += 1;
                }
            }
        }
    }

    println!("{}.", count);
}
