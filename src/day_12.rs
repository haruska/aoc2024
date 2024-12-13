use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Point(usize, usize);

impl Point {
    fn adjacent(&self, other: &Point) -> bool {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) == 1
    }
}

#[derive(Default, Clone, Debug)]
struct Map {
    plots: Vec<Vec<Plot>>,
    region_map: HashMap<char, Vec<Region>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let plots: Vec<Vec<Plot>> = input
            .lines()
            .enumerate()
            .map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .map(|(j, c)| Plot {
                        id: c,
                        position: Point(i, j),
                    })
                    .collect()
            })
            .collect();

        let region_map = plots
            .iter()
            .flatten()
            .fold(HashMap::new(), |mut acc, plot| {
                let entry: &mut Vec<Region> = acc.entry(plot.id).or_default();
                if let Some(region) = entry.iter_mut().find(|r| r.adjacent_plot(&plot)) {
                    region.add_plot(plot.clone())
                } else {
                    let mut region = Region {
                        plot_id: plot.id,
                        ..Default::default()
                    };
                    region.add_plot(plot.clone());
                    entry.push(region)
                }
                acc
            });

        Map { plots, region_map }
    }

    fn dimension(&self) -> (usize, usize) {
        (self.plots.len(), self.plots[0].len())
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Plot {
    id: char,
    position: Point,
}

impl Plot {
    fn adjacent(&self, other: &Plot) -> bool {
        self.position.adjacent(&other.position)
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
struct Region {
    plot_id: char,
    plots: HashSet<Plot>,
}

impl Region {
    fn add_plot(&mut self, plot: Plot) {
        assert_eq!(
            plot.id, self.plot_id,
            "Attempted to add plot with non-matching id to region"
        );
        self.plots.insert(plot);
    }

    fn adjacent_plot(&self, plot: &Plot) -> bool {
        self.plots.iter().any(|p| p.adjacent(&plot))
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.plots
            .iter()
            .map(|p| {
                let adjacent_count = self.plots.iter().filter(|other| other.adjacent(p)).count();
                4 - adjacent_count
            })
            .sum()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Map {
    Map::new(input)
}

#[aoc(day12, part1)]
fn part1(map: &Map) -> usize {
    map.region_map.values().flatten().map(|r| r.price()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const INPUT_ONE: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const INPUT_TWO: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};

    const INPUT_THREE: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn test_input_generator() {
        let map = input_generator(INPUT_ONE);
        assert_eq!(map.dimension(), (4, 4));
    }

    #[rstest]
    #[case::region_a('A', 4, 10)]
    #[case::region_b('B', 4, 8)]
    #[case::region_c('C', 4, 10)]
    #[case::region_d('D', 1, 4)]
    #[case::region_e('E', 3, 8)]
    fn test_map_one_regions(
        #[case] plot_id: char,
        #[case] exp_area: usize,
        #[case] exp_perimeter: usize,
    ) {
        let map = input_generator(INPUT_ONE);

        let region_list = map
            .region_map
            .get(&plot_id)
            .expect(format!("should be a region {plot_id}").as_str());

        assert_eq!(region_list.len(), 1);

        let region = &region_list[0];

        assert_eq!(region.area(), exp_area);
        assert_eq!(region.perimeter(), exp_perimeter);
    }

    #[test]
    fn test_region_holes() {
        let map = input_generator(INPUT_TWO);

        let count: usize = map.region_map.values().map(|r| r.len()).sum();
        assert_eq!(count, 5);

        let region = &map.region_map.get(&'O').unwrap()[0];
        assert_eq!(region.area(), 21);
        assert_eq!(region.perimeter(), 36);
    }

    #[rstest]
    #[case::one(0, 140)]
    #[case::two(1, 772)]
    #[case::three(2, 1930)]
    fn test_part1(#[case] i: usize, #[case] exp_price: usize) {
        let input = [INPUT_ONE, INPUT_TWO, INPUT_THREE][i];
        let map = input_generator(input);
        let result = part1(&map);
        assert_eq!(result, exp_price);
    }
}
