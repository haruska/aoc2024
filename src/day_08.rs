use aoc_runner_derive::aoc_generator;
use std::collections::{HashMap, HashSet};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Point(i32, i32);

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct MapLocation(usize, usize);

#[derive(Default, Clone, Debug)]
struct AntennaMap {
    dimensions: (usize, usize),
    antennas: HashMap<char, HashSet<MapLocation>>,
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
                let mut points = ant_map.antennas.entry(c).or_default();
                points.insert(MapLocation(i, j));
            }
        })
    });

    ant_map
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
}
