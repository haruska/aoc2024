use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Stone(u64);

impl Stone {
    fn transform(&self) -> Vec<Stone> {
        let val = self.0;

        if val == 0 {
            return vec![Stone(1)];
        }

        let digits = val.to_string();
        if digits.len() % 2 == 0 {
            let digits = digits.as_str();
            let mid_idx = digits.len() / 2;
            return vec![
                Stone(digits[..mid_idx].parse().unwrap()),
                Stone(digits[mid_idx..].parse().unwrap()),
            ];
        }

        vec![Stone(val * 2024)]
    }
}

type StoneStep = (Stone, usize);

#[derive(Default, PartialEq, Clone, Debug)]
struct TransformCache(HashMap<StoneStep, usize>);

impl TransformCache {
    fn blink_times(&mut self, n: usize, stone: &Stone) -> usize {
        let k: StoneStep = (stone.clone(), n);
        if let Some(n) = self.0.get(&k) {
            return *n;
        }

        let stones = stone.transform();

        let val = if n == 1 {
            stones.len()
        } else {
            stones.iter().map(|s| self.blink_times(n - 1, s)).sum()
        };

        self.0.insert(k, val);
        val
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|n| Stone(n.parse().unwrap()))
        .collect()
}

#[aoc(day11, part1)]
fn part1(stones: &[Stone]) -> usize {
    let mut cache = TransformCache::default();
    stones.iter().map(|s| cache.blink_times(25, s)).sum()
}

#[aoc(day11, part2)]
fn part2(stones: &[Stone]) -> usize {
    let mut cache = TransformCache::default();
    stones.iter().map(|s| cache.blink_times(75, s)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input, vec![Stone(125), Stone(17)]);
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 65601038650482);
    }
}
