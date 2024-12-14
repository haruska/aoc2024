#![allow(dead_code)]

use good_lp::{constraint, default_solver, variable, ProblemVariables, Solution, SolverModel};

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

struct ButtonPressCost {
    a: usize,
    b: usize,
    cost: usize,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_one() {
        let machine = Machine {
            button_a: Pair::new(94, 34),
            button_b: Pair::new(22, 67),
            prize: Pair::new(8400, 5400),
        };

        let res = machine.solve().expect("Failed to find solution");
        assert_eq!(res.a, 80);
        assert_eq!(res.b, 40);
        assert_eq!(res.cost, 280);
    }

    #[test]
    fn test_puzzle_two() {
        let machine = Machine {
            button_a: Pair::new(26, 66),
            button_b: Pair::new(67, 21),
            prize: Pair::new(12748, 12176),
        };

        assert!(machine.solve().is_none());
    }
}
