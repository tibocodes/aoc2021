const INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

fn main() {
    dbg!(part_1(INPUT));
    dbg!(part_1(include_str!("../../inputs/day8")));
}

fn part_1(input: &str) -> u32 {
    count_1_4_7_8(parsed_output(input))
}

fn parsed_output(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split('|')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(String::from)
                .collect()
        })
        .collect()
}

fn count_1_4_7_8(output: Vec<Vec<String>>) -> u32 {
    output.iter().fold(0, |count, line| {
        count
            + line
                .iter()
                .fold(0, |line_count, element| match element.len() {
                    2 | 4 | 3 | 7 => line_count + 1,
                    _ => line_count,
                })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 26);
    }
}
