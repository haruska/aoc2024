use aoc_runner_derive::aoc_generator;

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Calibration {
    total: u32,
    values: Vec<u32>,
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
}
