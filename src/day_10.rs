use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Point(i32, i32);

#[derive(Default, Clone, Debug)]
struct TgMap(Vec<Vec<u8>>);

impl TgMap {
    fn on_map(&self, point: &Point) -> bool {
        let (i, j) = (point.0, point.1);
        let (max_i, max_j) = self.dimensions();
        i >= 0 && i < max_i as i32 && j >= 0 && j < max_j as i32
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn trailheads(&self) -> Vec<Point> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &cell)| {
                    if cell == 0 {
                        Some(Point(i as i32, j as i32))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn at(&self, pos: &Point) -> u8 {
        self.0[pos.0 as usize][pos.1 as usize]
    }

    fn find_nines(&self, pos: &Point) -> HashSet<Point> {
        let val = self.at(pos);

        if val == 9 {
            return HashSet::from([pos.clone()]);
        }

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .fold(HashSet::new(), |mut acc, (idx, jdx)| {
                let neighbor = Point(pos.0 + idx, pos.1 + jdx);
                if self.on_map(&neighbor) && self.at(&neighbor) == val + 1 {
                    acc.extend(self.find_nines(&neighbor));
                }
                acc
            })
    }

    fn scores(&self) -> HashMap<Point, usize> {
        self.trailheads()
            .iter()
            .map(|th| (th.clone(), self.find_nines(th).len()))
            .collect()
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> TgMap {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    TgMap(map)
}

#[aoc(day10, part1)]
fn part1(tg_map: &TgMap) -> usize {
    tg_map.scores().values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.0.len(), 8);
        assert_eq!(input.0[0].len(), 8);
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 36);
    }
}
