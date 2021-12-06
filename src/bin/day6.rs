const INPUT: &str = "3,4,3,1,2";

fn main() {
    dbg!(part_1(INPUT, 80));
    dbg!(part_1(include_str!("../../inputs/day6"), 80));
}

fn part_1(input: &str, days: u32) -> usize {
    let mut fishes = parse(input);

    for _day in 0..days {
        // dbg!(_day);
        // dbg!(&fishes);
        let mut new_fishes = 0;
        fishes = fishes
                .into_iter()
                .map(|fish| {
                    if (fish - 1) >= 0 {
                        fish - 1
                    } else {
                        new_fishes += 1;
                        6
                    }
                })
                .collect::<Vec<i32>>();
        // dbg!(new_fishes);

        // TODO: probably a mor efficient way to add elements at once
        for _i in 0..new_fishes {
            fishes.push(8);
        }
    }
    fishes.len()
}

fn parse(input: &str) -> Vec<i32> {
    input.split(',').map(|e| e.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // assert_eq!(part_1(INPUT, 1), 5);
        // assert_eq!(part_1(INPUT, 2), 6);
        // assert_eq!(part_1(INPUT, 3), 7);
        // assert_eq!(part_1(INPUT, 4), 9);
        // assert_eq!(part_1(INPUT, 5), 10);
        assert_eq!(part_1(INPUT, 80), 5934);
    }
}
