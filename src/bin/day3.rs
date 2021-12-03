const INPUT: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

fn main() {
  println!("{:?}", vectors(INPUT));
  dbg!(gamma_rate(INPUT));
  dbg!(epsilon_rate(INPUT));
  dbg!(part_1(INPUT));
  dbg!(part_1(include_str!("../../inputs/day3")));
}

pub fn part_1(input: &str) -> u32 {
  gamma_rate(input) * epsilon_rate(input)
}

// Most common bits
fn gamma_rate(input: &str) -> u32 {
  let parsed_input = vectors(input);
  let binary_string = parsed_input
    .iter()
    .fold(vec![0; parsed_input.first().unwrap().len()], add)
    .iter()
    .map( |bit|
      if *bit as usize >= parsed_input.len() / 2 {
        1
      } else {
        0
      }
    )
    .collect::<Vec<u32>>()
    .iter()
    .map( |elem| elem.to_string())
    .collect::<Vec<String>>()
    .join("");

  u32::from_str_radix(&binary_string, 2).unwrap()
}

// Least common bits
fn epsilon_rate(input: &str) -> u32 {
  let parsed_input = vectors(input);
  let binary_string = parsed_input
    .iter()
    .fold(vec![0; parsed_input.first().unwrap().len()], add)
    .iter()
    .map( |bit|
      if *bit as usize >= parsed_input.len() / 2 {
        0
      } else {
        1
      }
    )
    .collect::<Vec<u32>>()
    .iter()
    .map( |elem| elem.to_string())
    .collect::<Vec<String>>()
    .join("");

  u32::from_str_radix(&binary_string, 2).unwrap()
}

fn vectors(input: &str) -> Vec<Vec<u32>>{
  input
  .lines()
  .filter(|line| !line.is_empty())
  .map(|line|
    line
    .chars()
    .map(|e| e.to_digit(10).unwrap())
    .collect()
  )
  .collect()
}

pub fn add(vec1: Vec<u32>, vec2: &Vec<u32>) -> Vec<u32> {
  vec1.iter().zip(&*vec2).map(|elem| elem.0 + elem.1).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

//     const INPUT: &str = "
// 00100
// 11110
// 10110
// 10111
// 10101
// 01111
// 00111
// 11100
// 10000
// 11001
// 00010
// 01010
// ";

    #[test]
    fn exploration() {
      assert_eq!(part_1(INPUT), 198);
    }
    
    #[test]
    fn test_add() {
      assert_eq!(add(vec![0, 1], &vec![1, 0]), vec![1, 1])
    }
}
