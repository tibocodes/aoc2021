fn main() {
    // dbg!(numbers(INPUT));
    // dbg!(Grid::new(GRID_INPUT));
    dbg!(Grid::new(GRID_INPUT).won(&vec![22, 13, 17, 11, 0]));
    dbg!(Grid::new(GRID_INPUT).won(&vec![22, 13, 17, 11, 1]));
    dbg!(Grid::new(GRID_INPUT).won(&vec![22, 8, 21, 6, 1]));
    dbg!(Grid::new(GRID_INPUT).won(&vec![22, 9, 21, 6, 1]));
}

const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

const GRID_INPUT: &str = "22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19
";

fn numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Grid {
    // TODO: should use a fixed length array since we know there will be 25 elements
    numbers: Vec<u32>,
}

impl Grid {
    fn new(input_str: &str) -> Grid {
        input_str
            .split_whitespace()
            .fold(Grid { numbers: vec![] }, |mut grid, elem| {
                grid.numbers.push(elem.parse().unwrap());
                grid
            })
    }

    fn won(&self, played_numbers: &Vec<u32>) -> bool {
        self.won_horizontally(played_numbers)
        // self.won_horizontally(played_numbers) || self.won_vertically(&played_numbers)
    }

    fn won_horizontally(&self, played_numbers: &Vec<u32>) -> bool {
        self.numbers
            .windows(5)
            .any(|line| line.iter().all(|e| played_numbers.contains(e)))
    }

    fn won_vertically(&self, played_numbers: &Vec<u32>) -> bool {
        (0..=4).any(|start| {
            self.numbers
                .iter()
                .skip(start)
                .step_by(5)
                .all(|e| played_numbers.contains(e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_won_horizontally() {
        assert_eq!(Grid::new(GRID_INPUT).won_horizontally(&vec![22, 13, 17, 11, 0]), true);
        assert_eq!(Grid::new(GRID_INPUT).won_horizontally(&vec![22, 13, 17, 11, 1]), false);
    }

    #[test]
    fn test_won_vertically() {
        assert_eq!(Grid::new(GRID_INPUT).won_vertically(&vec![22, 8, 21, 6, 1]), true);
        assert_eq!(Grid::new(GRID_INPUT).won_vertically(&vec![22, 9, 21, 6, 1]), false);
    }
}
