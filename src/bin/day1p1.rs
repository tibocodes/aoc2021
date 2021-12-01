use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let f = File::open("inputs/day1/input")?;
    let f = io::BufReader::new(f);

    let mut prev: Option<i32> = None;
    let mut count = 0;
    for line in f.lines() {
        let current: i32 = line.unwrap().parse().unwrap();

        if let Some(x) = prev {
            if current > x {
                count += 1;
            }
        }
        prev = Some(current);
    }
    println!("count: {}", count);
    Ok(())
}
