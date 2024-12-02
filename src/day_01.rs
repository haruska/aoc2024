use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::zip;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let mut first: Vec<u32> = pairs.iter().map(|(a, _b)| *a).collect();
    let mut second: Vec<u32> = pairs.iter().map(|(_a, b)| *b).collect();

    first.sort();
    second.sort();

    (first, second)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = input;
    zip(left, right)
        .map(|(l, r)| {
            let diff: i32 = *l as i32 - *r as i32;
            diff.unsigned_abs()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    fn counts(nums: &[u32]) -> HashMap<u32, u32> {
        nums.iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(*x).and_modify(|cnt| *cnt += 1).or_insert(1);
            acc
        })
    }

    let left = counts(input.0.as_slice());
    let right = counts(input.1.as_slice());

    left.iter()
        .map(|(val, times)| {
            let cnt = right.get(val).unwrap_or(&0);
            val * times * cnt
        })
        .sum()
}
