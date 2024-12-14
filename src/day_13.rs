#![allow(dead_code)]

use aoc_runner_derive::{aoc, aoc_generator};
use good_lp::{constraint, default_solver, variable, ProblemVariables, Solution, SolverModel};
use regex::Regex;

#[derive(Default, PartialEq, Clone, Debug)]
struct Pair {
    x: usize,
    y: usize,
}

impl Pair {
    fn new(x: usize, y: usize) -> Self {
        Pair { x, y }
    }
    fn x_f64(&self) -> f64 {
        self.x as f64
    }

    fn y_f64(&self) -> f64 {
        self.y as f64
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
struct ButtonPressCost {
    a: usize,
    b: usize,
    cost: usize,
}

#[derive(Default, PartialEq, Clone, Debug)]
struct Machine {
    button_a: Pair,
    button_b: Pair,
    prize: Pair,
}

impl Machine {
    fn solve(&self) -> Option<ButtonPressCost> {
        let mut problem = ProblemVariables::new();

        let a = problem.add(variable().integer().min(0)); // Integer variable for `a`
        let b = problem.add(variable().integer().min(0)); // Integer variable for `b`

        problem
            .minimise(3 * a + b) // Minimize the cost function C = 3a + b
            .using(default_solver)
            .with(constraint!(
                self.button_a.x_f64() * a + self.button_b.x_f64() * b == self.prize.x_f64()
            ))
            .with(constraint!(
                self.button_a.y_f64() * a + self.button_b.y_f64() * b == self.prize.y_f64()
            ))
            .solve()
            .map(|solution| {
                let a_value = solution.value(a) as usize;
                let b_value = solution.value(b) as usize;
                let cost = 3 * a_value + b_value;

                ButtonPressCost {
                    a: a_value,
                    b: b_value,
                    cost,
                }
            })
            .ok()
    }
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    input
        .split("\n\n")
        .map(|m_input| {
            let mut parsed = m_input.lines().map(|l| {
                let res = re.captures(l).unwrap();
                Pair::new(
                    res.get(1).unwrap().as_str().parse().unwrap(),
                    res.get(2).unwrap().as_str().parse().unwrap(),
                )
            });

            Machine {
                button_a: parsed.next().unwrap(),
                button_b: parsed.next().unwrap(),
                prize: parsed.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|m| m.solve())
        .map(|sol| sol.cost)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);

        assert_eq!(input.len(), 4);

        let exp = Machine {
            button_a: Pair::new(94, 34),
            button_b: Pair::new(22, 67),
            prize: Pair::new(8400, 5400),
        };

        assert_eq!(input[0], exp);

        let exp = Machine {
            button_a: Pair::new(26, 66),
            button_b: Pair::new(67, 21),
            prize: Pair::new(12748, 12176),
        };

        assert_eq!(input[1], exp);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(TEST_INPUT);
        let res = part1(input.as_slice());
        assert_eq!(res, 480);
    }

    #[test]
    fn test_puzzle_one() {
        let machine = input_generator(TEST_INPUT)[0].clone();

        let res = machine.solve().expect("Failed to find solution");
        assert_eq!(res.a, 80);
        assert_eq!(res.b, 40);
        assert_eq!(res.cost, 280);
    }

    #[test]
    fn test_puzzle_two() {
        let machine = input_generator(TEST_INPUT)[1].clone();
        assert!(machine.solve().is_none());
    }

    #[test]
    fn test_puzzle_three() {
        let machine = input_generator(TEST_INPUT)[2].clone();

        let res = machine.solve().expect("Failed to find solution");
        assert_eq!(res.a, 38);
        assert_eq!(res.b, 86);
        assert_eq!(res.cost, 200);
    }

    #[test]
    fn test_puzzle_four() {
        let machine = input_generator(TEST_INPUT)[3].clone();
        assert!(machine.solve().is_none());
    }
}
