use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Point(i32, i32);

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }

    fn subtract(&self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl From<&MapLocation> for Point {
    fn from(value: &MapLocation) -> Self {
        Point(value.0 as i32, value.1 as i32)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct MapLocation(usize, usize);

impl MapLocation {
    fn antinode(&self, other: &Self) -> Point {
        let diff = self.diff(other);
        Point::from(self).add(&diff)
    }

    fn diff(&self, other: &Self) -> Point {
        Point::from(self).subtract(&other.into())
    }
}

#[derive(Default, Clone, Debug)]
struct AntennaMap {
    dimensions: (usize, usize),
    antennas: HashMap<char, HashSet<MapLocation>>,
}

impl AntennaMap {
    fn on_map(&self, point: &Point) -> bool {
        let (i, j) = (point.0, point.1);
        let (max_i, max_j) = self.dimensions;
        i >= 0 && i < max_i as i32 && j >= 0 && j < max_j as i32
    }

    fn location(&self, point: &Point) -> Option<MapLocation> {
        if self.on_map(point) {
            Some(MapLocation(point.0 as usize, point.1 as usize))
        } else {
            None
        }
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> AntennaMap {
    let lines: Vec<&str> = input.lines().collect();
    let mut ant_map = AntennaMap {
        dimensions: (lines.len(), lines[0].len()),
        ..Default::default()
    };

    lines.into_iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '.' => {}
            _ => {
                let points = ant_map.antennas.entry(c).or_default();
                points.insert(MapLocation(i, j));
            }
        })
    });

    ant_map
}

#[aoc(day8, part1)]
fn part1(ant_map: &AntennaMap) -> usize {
    ant_map
        .antennas
        .values()
        .fold(HashSet::new(), |mut acc, ant_loc| {
            let locations: Vec<MapLocation> = ant_loc.iter().cloned().collect();
            let locations = locations.as_slice();

            locations.iter().enumerate().for_each(|(i, head)| {
                let tail = &locations[i + 1..];
                if !tail.is_empty() {
                    tail.iter().for_each(|other| {
                        if let Some(antinode) = ant_map.location(&head.antinode(other)) {
                            acc.insert(antinode);
                        }
                        if let Some(antinode) = ant_map.location(&other.antinode(head)) {
                            acc.insert(antinode);
                        }
                    });
                }
            });
            acc
        })
        .len()
}

#[aoc(day8, part2)]
fn part2(ant_map: &AntennaMap) -> usize {
    ant_map
        .antennas
        .values()
        .fold(HashSet::new(), |mut acc, ant_loc| {
            let locations: Vec<MapLocation> = ant_loc.iter().cloned().collect();
            let locations = locations.as_slice();

            locations.iter().enumerate().for_each(|(i, head)| {
                let tail = &locations[i + 1..];
                if !tail.is_empty() {
                    tail.iter().for_each(|other| {
                        let diff = head.diff(other);
                        let mut next = Point::from(head);
                        acc.insert(head.clone());
                        while let Some(antinode) = ant_map.location(&next.add(&diff)) {
                            acc.insert(antinode);
                            next = next.add(&diff);
                        }

                        let diff = other.diff(head);
                        let mut next = Point::from(other);
                        acc.insert(other.clone());
                        while let Some(antinode) = ant_map.location(&next.add(&diff)) {
                            acc.insert(antinode);
                            next = next.add(&diff);
                        }
                    });
                }
            });
            acc
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.antennas.len(), 2);
        assert!(input.antennas[&'A'].contains(&MapLocation(8, 8)));
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 34);
    }
}
