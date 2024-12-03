use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3, part1)]
pub fn input_generator_one(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        .collect()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_two(input: &str) -> Vec<(u32, u32)> {
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let mut res = vec![];

    match dont_re.find(input) {
        None => res.extend(input_generator_one(input)),
        Some(m) => {
            res.extend(input_generator_one(&input[..m.start()]));

            let remaining = &input[m.end()..];
            if let Some(m2) = do_re.find(remaining) {
                res.extend(input_generator_two(&remaining[m2.end()..]));
            }
        }
    }

    res
}

#[aoc(day3, part1)]
#[aoc(day3, part2)]
pub fn solution(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_input_generator_one() {
        let res = input_generator_one(TEST_INPUT);
        let exp = vec![(2, 4), (5, 5), (11, 8), (8, 5)];

        assert_eq!(res, exp);
    }

    #[test]
    fn test_input_generator_two() {
        let res = input_generator_two(TEST_INPUT);
        let exp = vec![(2, 4), (8, 5)];

        assert_eq!(res, exp);
    }

    #[test]
    fn test_part_one() {
        let input = input_generator_one(TEST_INPUT);
        let result = solution(input.as_slice());

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator_two(TEST_INPUT);
        let result = solution(input.as_slice());

        assert_eq!(result, 48);
    }
}
