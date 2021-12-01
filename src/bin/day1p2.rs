use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let f = File::open("inputs/day1/input")?;
    let f = io::BufReader::new(f);

    let mut count = 0;
    let mut vector_lines = Vec::new();
    for line in f.lines() {
        let value: i32 = line.unwrap().parse().unwrap();
        vector_lines.push(value);
    }

    let mut prev: Option<i32> = None;
    for window in vector_lines.windows(3) {
        let sum: i32 = window.iter().sum();
        if let Some(x) = prev {
            if sum > x {
                count += 1;
            }
        }
        prev = Some(sum);
    }
    println!("count: {}", count);
    Ok(())
}
