use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct MapLocation {
    point: Point,
}

impl MapLocation {
    fn off_map(&self) -> bool {
        let (i, j) = self.point;
        i < 0 || j < 0
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
enum Direction {
    #[default]
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn offset(&self) -> Point {
        match self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
struct GuardPosition {
    location: MapLocation,
    direction: Direction,
}

#[derive(Default, Clone, Debug)]
struct ObstructionMap {
    obstructions: HashSet<MapLocation>,
    guard_position: GuardPosition,
}

impl ObstructionMap {
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
    let mut ob_map = ObstructionMap::default();

    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '#' => ob_map.insert((i, j)),
            '^' => ob_map.start_guard((i, j)),
            _ => {}
        })
    });

    ob_map
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
}
