use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Page {
    num: u32,
    before: HashSet<u32>,
    after: HashSet<u32>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PageList {
    pages: Vec<Page>,
}

impl PageList {
    pub fn new(nums: &[u32]) -> Self {
        let mut p_list = PageList::default();
        nums.iter().for_each(|n| p_list.add(*n));
        p_list
    }

    fn add(&mut self, num: u32) {
        //insert num into each previous page
        //and calculate current page's before set
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
pub fn input_generator(input: &str) -> (HashMap<u32, Page>, Vec<PageList>) {
    let (input1, input2) = split_once(input, "\n\n");

    let page_map = input1.lines().fold(HashMap::new(), |mut map, line| {
        let (before_s, after_s) = split_once(line, "|");
        let before = before_s.parse().unwrap();
        let after = after_s.parse().unwrap();

        let mut page = map.entry(before).or_insert(Page {
            num: before,
            ..Default::default()
        });
        page.after.insert(after);

        let mut page = map.entry(after).or_insert(Page {
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

fn split_once<'a>(in_string: &'a str, pat: &str) -> (&'a str, &'a str) {
    let mut splitter = in_string.splitn(2, pat);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
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
}
