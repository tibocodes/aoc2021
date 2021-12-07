const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

use std::collections::HashMap;

fn main() {
    dbg!(part_1(include_str!("../../inputs/day7")));
}

fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|e| e.parse::<i32>().unwrap()).collect()
}
fn part_1(input: &str) -> i32 {
    let parsed_input = parse(input);
    let min = parsed_input.iter().min().unwrap();
    let max = parsed_input.iter().max().unwrap();
    let mut results: HashMap<i32, i32> = HashMap::new();
    for position in *min..=*max {
        results.insert(position, calculate_fuel(&parsed_input, position));
    }
    *results.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1
}

fn calculate_fuel(parsed_input: &Vec<i32>, position: i32) -> i32 {
    parsed_input
        .iter()
        .fold(0, |acc, elem| acc + (elem - position).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 37);
    }
}
