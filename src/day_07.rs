use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Calibration {
    total: usize,
    values: Vec<usize>,
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

fn possible(total: usize, is_part2: bool, acc: usize, values: &[usize]) -> bool {
    if values.is_empty() {
        return total == acc;
    }
    if acc > total {
        return false;
    }

    let concat_possible = is_part2
        && possible(
            total,
            is_part2,
            format!("{}{}", acc, values[0]).parse().unwrap(),
            &values[1..],
        );

    concat_possible
        || possible(total, is_part2, acc * values[0], &values[1..])
        || possible(total, is_part2, acc + values[0], &values[1..])
}

fn solve(input: &[Calibration], is_part2: bool) -> usize {
    input
        .iter()
        .filter(|c| possible(c.total, is_part2, 0, c.values.as_slice()))
        .map(|c| c.total)
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &[Calibration]) -> usize {
    solve(input, false)
}

#[aoc(day7, part2)]
fn part2(input: &[Calibration]) -> usize {
    solve(input, true)
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
