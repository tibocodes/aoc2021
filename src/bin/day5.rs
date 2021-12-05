const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

fn main() {
    dbg!(parse_input(INPUT));
}

#[derive(Debug)]
struct Grid {
    state: [[u32; 10]; 10],
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    // Didn't manage to make it a [usize; 2], the collect wouldn't work then
    fn new(coords: Vec<usize>) -> Point {
        Point {
            x: coords[0],
            y: coords[1],
        }
    }
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            state: [[0; 10]; 10],
        }
    }
    fn add_line() {}
}

// fn parse_input(input: &str) -> Vec<Vec<Point>> {
fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    Point::new(
                        coords
                            .split(',')
                            .map(|number| number.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .collect::<Vec<Point>>()
        })
        .map(|vec| (vec[0], vec[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_line() {
        assert_eq!();
    }
}
