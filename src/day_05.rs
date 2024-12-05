use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type RuleSet = HashMap<u32, Page>;

#[derive(Debug, PartialEq, Clone, Default)]
struct Page {
    num: u32,
    before: HashSet<u32>,
    after: HashSet<u32>,
}

impl Page {
    fn valid(&self, rules: &RuleSet) -> bool {
        let p_rules = rules.get(&self.num).unwrap();
        self.before.intersection(&p_rules.after).count() == 0
            && self.after.intersection(&p_rules.before).count() == 0
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PageList {
    pages: Vec<Page>,
}

impl PageList {
    fn new(nums: &[u32]) -> Self {
        let mut p_list = PageList::default();
        nums.iter().for_each(|n| p_list.add(*n));
        p_list
    }

    fn valid(&self, rules: &RuleSet) -> bool {
        self.pages.iter().all(|page| page.valid(rules))
    }

    fn page_nums(&self) -> Vec<u32> {
        self.pages.iter().map(|p| p.num).collect()
    }

    fn middle_page(&self) -> u32 {
        let idx = self.pages.len() / 2;
        self.pages[idx].num
    }

    fn sorted(&self, rules: &RuleSet) -> PageList {
        let mut ordered: Vec<u32> = Vec::with_capacity(self.pages.len());

        self.page_nums().iter().enumerate().for_each(|(i, num)| {
            ordered.push(*num);
            let mut idx = i;
            while !PageList::new(ordered.as_slice()).valid(rules) {
                ordered.swap(idx, idx - 1);
                idx -= 1;
            }
        });

        PageList::new(ordered.as_slice())
    }

    fn add(&mut self, num: u32) {
        // insert num into each previous page and calculate current page's before set
        let before = self
            .pages
            .iter_mut()
            .map(|page| {
                page.after.insert(num);
                page.num
            })
            .collect();

        let page = Page {
            num,
            before,
            ..Default::default()
        };

        self.pages.push(page);
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (RuleSet, Vec<PageList>) {
    let (input1, input2) = input.split_once("\n\n").unwrap();

    let page_map = input1.lines().fold(HashMap::new(), |mut map, line| {
        let (before_s, after_s) = line.split_once("|").unwrap();
        let before = before_s.parse().unwrap();
        let after = after_s.parse().unwrap();

        let page = map.entry(before).or_insert(Page {
            num: before,
            ..Default::default()
        });
        page.after.insert(after);

        let page = map.entry(after).or_insert(Page {
            num: after,
            ..Default::default()
        });
        page.before.insert(before);

        map
    });

    let page_lists: Vec<PageList> = input2
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l.split(',').map(|x| x.parse().unwrap()).collect();
            PageList::new(nums.as_slice())
        })
        .collect();

    (page_map, page_lists)
}

#[aoc(day5, part1)]
fn part1(input: &(RuleSet, Vec<PageList>)) -> u32 {
    let (rules, updates) = input;
    updates
        .iter()
        .filter(|pl| pl.valid(rules))
        .map(|pl| pl.middle_page())
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &(RuleSet, Vec<PageList>)) -> u32 {
    let (rules, updates) = input;
    updates
        .iter()
        .filter(|pl| !pl.valid(rules))
        .map(|pl| pl.sorted(rules).middle_page())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn test_input_generator() {
        let (map, list) = input_generator(TEST_INPUT);
        assert_eq!(map.len(), 7);
        assert_eq!(list.len(), 6);
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 123);
    }
}
