fn main() {
  part_1();
}

fn inputs() -> Vec<Move<'static>> {
  const INPUT: &str = include_str!("../../inputs/day2");
  let moves: Vec<Move> = INPUT
      .lines()
      .map(|line| {
          let mut split = line.split(' ');
          Move(
              split.next().unwrap(),
              split.next().unwrap().parse().unwrap(),
          )
      })
      .collect();
  moves
}

fn part_1() {
  let mut position = Position(0, 0);
  for m in inputs().iter() {
    match m.0 {
      "forward" => position.0 += m.1,
      "down" => position.1 += m.1,
      "up" => position.1 -= m.1,
      _ => panic!("invalid input")
    }
  }
  let result = position.0 * position.1;
  dbg!(result);
}

#[derive(Debug)]
struct Move<'a>(&'a str, i32);

#[derive(Debug)]
struct Position(i32, i32);