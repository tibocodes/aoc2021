use std::cmp;

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
    // part_1(INPUT);
    part_1(include_str!("../../inputs/day5"));
}

fn part_1(input: &str) -> u32 {
    let mut grid = Grid::new(1000);
    for line in parse_input(input) {
        let line = Line::new(line);
        // dbg!(line);
        grid.add_line(line);
    }
    // dbg!(grid);
    dbg!(grid.count_above(2));
    grid.count_above(2)
}

#[derive(Debug)]
struct Grid {
    state: [[u32; 1000]; 1000],
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

#[derive(Debug)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn new(points: (Point, Point)) -> Line {
        // dbg!(points);
        let mut all_points = vec![];
        if points.0.x == points.1.x {
            let y_min = cmp::min(points.0.y, points.1.y);
            let y_max = cmp::max(points.0.y, points.1.y);
            for y in y_min..=y_max {
                all_points.push(Point {
                    x: points.0.x,
                    y,
                });
            }
        } else if points.0.y == points.1.y {
            let x_min = cmp::min(points.0.x, points.1.x);
            let x_max = cmp::max(points.0.x, points.1.x);
            for x in x_min..=x_max {
                all_points.push(Point {
                    x,
                    y: points.0.y,
                });
            }
        }
        Line { points: all_points }
    }
}

impl Grid {
    // TODO: fix
    fn new(_size: usize) -> Grid {
        Grid {
            state: [[0; 1000]; 1000],
        }
    }
    fn add_line(&mut self, line: Line) {
        for point in line.points.into_iter() {
            self.state[point.x][point.y] += 1;
        }
    }

    fn count_above(&self, value: u32) -> u32 {
        // rewrite with fold
        let mut count = 0;
        for line in self.state.iter() {
            for val in line.iter() {
                if *val >= value {
                    count += 1;
                }
            }
        }
        count
    }
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
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 5)
    }
}
