use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_input_generator() {
        let res = input_generator(TEST_INPUT);
        let exp = vec![(2, 4), (5, 5), (11, 8), (8, 5)];

        assert_eq!(res, exp);
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(input.as_slice());

        assert_eq!(result, 161);
    }
}
