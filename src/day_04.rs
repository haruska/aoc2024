use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Puzzle {
    chars: Vec<Vec<char>>,
}

impl Puzzle {
    pub fn new(input: &str) -> Self {
        let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Puzzle { chars }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<char> {
        self.chars.get(i).and_then(|row| row.get(j)).cloned()
    }

    pub fn find(&self, c: char) -> Vec<(usize, usize)> {
        self.chars
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(
                    move |(j, &ch)| {
                        if ch == c {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect()
    }

    fn check_direction(&self, offset: (i32, i32), start: (i32, i32), chars: &[char]) -> bool {
        if chars.is_empty() {
            return true;
        }
        let c = chars[0];

        let (idx, jdx) = offset;
        let (i, j) = start;

        if i < 0 || j < 0 {
            return false;
        }

        if let Some(c2) = self.get(i as usize, j as usize) {
            if c == c2 {
                self.check_direction(offset, (i + idx, j + jdx), &chars[1..])
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn check(&self, start: (usize, usize)) -> usize {
        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let chars: Vec<char> = "XMAS".chars().collect();

        let i = start.0 as i32;
        let j = start.1 as i32;

        directions
            .into_iter()
            .filter(|direction| self.check_direction(*direction, (i, j), chars.as_slice()))
            .count()
    }

    pub fn check_for_x(&self, a: (usize, usize)) -> bool {
        let (i, j) = a;
        if i == 0 || j == 0 {
            return false;
        }

        let chars1: String = [
            self.get(i - 1, j - 1).unwrap_or('Z'),
            self.get(i + 1, j + 1).unwrap_or('Z'),
        ]
        .iter()
        .collect();

        let chars2: String = [
            self.get(i - 1, j + 1).unwrap_or('Z'),
            self.get(i + 1, j - 1).unwrap_or('Z'),
        ]
        .iter()
        .collect();

        [chars1, chars2].iter().all(|cs| cs == "SM" || cs == "MS")
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Puzzle {
    Puzzle::new(input)
}

#[aoc(day4, part1)]
pub fn part1(input: &Puzzle) -> usize {
    input
        .find('X')
        .iter()
        .fold(0, |acc, point| acc + input.check(*point))
}

#[aoc(day4, part2)]
pub fn part2(input: &Puzzle) -> usize {
    input.find('A').iter().fold(0, |acc, point| {
        if input.check_for_x(*point) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn test_input_generator() {
        let puzzle = input_generator(TEST_INPUT);
        assert_eq!(puzzle.chars.len(), 10);
        assert_eq!(puzzle.chars[0].len(), 10);

        let s: String = puzzle.chars[0].iter().collect();
        assert_eq!(s.as_str(), "MMMSXXMASM");
    }

    #[test]
    fn test_get() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.get(2, 0), Some('A'));
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 9);
    }
}
