use aoc_runner_derive::aoc_generator;

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Stone(u64);

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|n| Stone(n.parse().unwrap()))
        .collect()
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
}
