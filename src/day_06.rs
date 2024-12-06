use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct MapLocation {
    point: Point,
}

impl MapLocation {
    fn location_at(&self, direction: &Direction) -> MapLocation {
        let (i, j) = self.point;
        let (idx, jdx) = direction.offset();
        MapLocation {
            point: (i + idx, j + jdx),
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> Point {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
struct GuardPosition {
    location: MapLocation,
    direction: Direction,
}

impl GuardPosition {
    fn rotate(&mut self) {
        self.direction = self.direction.rotate();
    }
}

#[derive(Default, Clone, Debug)]
struct ObstructionMap {
    dimensions: (usize, usize),
    obstructions: HashSet<MapLocation>,
    guard_position: GuardPosition,
}

impl ObstructionMap {
    fn off_map(&self, loc: &MapLocation) -> bool {
        let (i, j) = loc.point;
        let (max_i, max_j) = self.dimensions;

        i < 0 || j < 0 || i >= max_i as i32 || j >= max_j as i32
    }
    fn insert(&mut self, point: (usize, usize)) {
        let (i, j) = point;
        let location = MapLocation {
            point: (i as i32, j as i32),
        };
        self.obstructions.insert(location);
    }

    fn start_guard(&mut self, point: (usize, usize)) {
        let (i, j) = point;
        let location = MapLocation {
            point: (i as i32, j as i32),
        };
        self.guard_position = GuardPosition {
            location,
            ..Default::default()
        };
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> ObstructionMap {
    let lines: Vec<&str> = input.lines().collect();
    let mut ob_map = ObstructionMap {
        dimensions: (lines.len(), lines[0].len()),
        ..Default::default()
    };

    lines.into_iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '#' => ob_map.insert((i, j)),
            '^' => ob_map.start_guard((i, j)),
            _ => {}
        })
    });

    ob_map
}

#[aoc(day6, part1)]
fn part1(input: &ObstructionMap) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut pos = input.guard_position.clone();

    loop {
        visited.insert(pos.location.point);
        let loc = pos.location.location_at(&pos.direction);
        if input.off_map(&loc) {
            break;
        }

        if input.obstructions.contains(&loc) {
            pos.rotate();
        } else {
            pos.location = loc;
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.obstructions.len(), 8);
        assert_eq!(input.guard_position.location.point, (6, 4));
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 41);
    }
}
