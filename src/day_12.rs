use aoc_runner_derive::aoc_generator;
use std::collections::{HashMap, HashSet};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct Point(usize, usize);

#[derive(Default, Clone, Debug)]
struct Map {
    plots: Vec<Vec<Plot>>,
    region_map: HashMap<char, Vec<Region>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let plots = input
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

        Map {
            plots,
            ..Default::default()
        }
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

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        todo!()
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Map {
    Map::new(input)
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
    fn test_map_one(#[case] plot_id: char, #[case] exp_area: usize, #[case] exp_perimeter: usize) {
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
}
