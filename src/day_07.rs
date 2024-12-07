use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
struct Calibration {
    total: u64,
    values: Vec<u64>,
}

#[derive(PartialEq, Debug)]
enum Operator {
    Add,
    Mult,
    Concat,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Mult => a * b,
            Operator::Concat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

fn possible(total: u64, ops: &[Operator], acc: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return total == acc;
    }
    if acc > total {
        return false;
    }

    ops.iter()
        .any(|op| possible(total, ops, op.apply(acc, values[0]), &values[1..]))
}

fn solve(input: &[Calibration], ops: &[Operator]) -> u64 {
    input
        .iter()
        .filter(|c| possible(c.total, ops, 0, c.values.as_slice()))
        .map(|c| c.total)
        .sum()
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|l| {
            let (total_s, values_s) = l.split_once(": ").unwrap();
            Calibration {
                total: total_s.parse().unwrap(),
                values: values_s
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Calibration]) -> u64 {
    solve(input, &[Operator::Add, Operator::Mult])
}

#[aoc(day7, part2)]
fn part2(input: &[Calibration]) -> u64 {
    solve(input, &[Operator::Add, Operator::Mult, Operator::Concat])
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(), 9);

        let exp = Calibration {
            total: 161011,
            values: vec![16, 10, 13],
        };

        assert_eq!(input[5], exp)
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 11387);
    }
}
