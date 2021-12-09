const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

fn main() {
    dbg!(part_1(parsed_input(include_str!("../../inputs/day9"))));
}

fn parsed_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_1(parsed_input: Vec<Vec<u32>>) -> u32 {
    let mut lowpoints = vec![];
    for (x, line) in parsed_input.iter().enumerate() {
        for (y, elem) in line.into_iter().enumerate() {
            let mut min = true;
            for a in [-1, 1] as [i32; 2] {
                if let Some(adj_line) = parsed_input.get((x as i32 + a) as usize) {
                    if adj_line[y] <= *elem {
                        min = false;
                    }
                }
            }
            for b in [-1, 1]  as [i32; 2] {
                if let Some(adj_elem) = line.get((y as i32 + b) as usize) {
                    if *adj_elem <= *elem {
                        // should have a specific return
                        min = false;
                    }
                }
            }
            if min == true {
                // dbg!(x);
                // dbg!(y);
                // dbg!(elemv);
                lowpoints.push(elem + 1)
            }
        }
    }
    lowpoints.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(parsed_input(INPUT)), 15);
    }
}
