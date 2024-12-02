use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn all_increasing(&self) -> bool {
        self.levels.windows(2).all(|w| w[0] <= w[1])
    }

    fn all_decreasing(&self) -> bool {
        self.levels.windows(2).all(|w| w[0] >= w[1])
    }

    fn diff_min_max(&self) -> (u32, u32) {
        let diffs: Vec<u32> = self
            .levels
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .collect();
        let min = diffs.iter().min().unwrap();
        let max = diffs.iter().max().unwrap();

        (*min, *max)
    }

    pub fn safe(&self) -> bool {
        let sorted = self.all_decreasing() || self.all_increasing();
        let (min, max) = self.diff_min_max();

        sorted && min >= 1 && max <= 3
    }

    pub fn dampened_safe(&self) -> bool {
        if self.safe() {
            return true;
        }
        for i in 0..self.levels.len() {
            let mut r = self.clone();
            r.levels.remove(i);
            if r.safe() {
                return true;
            }
        }
        false
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| Report {
            levels: line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect(),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Report]) -> usize {
    input.iter().filter(|r| r.safe()).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Report]) -> usize {
    input.iter().filter(|r| r.dampened_safe()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn test_input_generator() {
        let reports = input_generator(TEST_INPUT);
        let exp: Vec<Report> = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];

        assert_eq!(reports, exp);
    }

    #[test]
    fn test_part_one() {
        let reports = input_generator(TEST_INPUT);
        let result = part1(reports.as_slice());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let reports = input_generator(TEST_INPUT);
        let result = part2(reports.as_slice());
        assert_eq!(result, 4);
    }
}
