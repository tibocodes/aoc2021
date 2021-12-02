fn main() {
  part_1();
  part_2();
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

fn part_2() {
  let mut position = Submarine{x: 0, y: 0, aim: 0};
  for m in inputs().iter() {
    match m.0 {
      "forward" => {
        position.x += m.1;
        position.y += position.aim * m.1;
      },
      "down" => position.aim += m.1,
      "up" => position.aim -= m.1,
      _ => panic!("invalid input")
    }
  }
  let result = position.x * position.y;
  dbg!(result);
}

#[derive(Debug)]
struct Move<'a>(&'a str, i32);

#[derive(Debug)]
struct Position(i32, i32);

#[derive(Debug)]
struct Submarine {
  x: i32,
  y: i32,
  aim: i32
}