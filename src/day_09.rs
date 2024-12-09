use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Formatter, Write};
use std::mem::discriminant;
use std::{fmt, iter};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum FileBlock {
    FileId(u32),
    FreeSpace,
}

impl fmt::Display for FileBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            FileBlock::FileId(id) => char::from_digit(*id % 36, 36).unwrap(),
            FileBlock::FreeSpace => '.',
        };

        f.write_char(c)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct DiskMap {
    blocks: Vec<FileBlock>,
}

impl fmt::Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.blocks.iter().map(|fb| fb.fmt(f)).collect()
    }
}

impl DiskMap {
    fn defrag(&mut self) {
        //advance cursor to first empty block
        let mut cursor = 0;
        while self.blocks[cursor] != FileBlock::FreeSpace {
            cursor += 1;
        }

        //advance end_cursor to last filled block
        let mut end_cursor = self.blocks.len() - 1;
        while self.blocks[end_cursor] == FileBlock::FreeSpace {
            end_cursor -= 1;
        }

        while cursor <= end_cursor {
            self.blocks.swap(cursor, end_cursor);

            while self.blocks[cursor] != FileBlock::FreeSpace {
                cursor += 1;
            }
            while self.blocks[end_cursor] == FileBlock::FreeSpace {
                end_cursor -= 1;
            }
        }
    }
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                FileBlock::FileId(fid) => i * *fid as usize,
                FileBlock::FreeSpace => 0,
            })
            .sum()
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> DiskMap {
    let blocks: Vec<FileBlock> = input.chars().enumerate().fold(vec![], |mut acc, (i, c)| {
        let x = c.to_string().parse().unwrap();

        let block = if i % 2 == 0 {
            // even index is a file id
            let file_id = (i / 2) as u32;
            FileBlock::FileId(file_id)
        } else {
            FileBlock::FreeSpace
        };

        acc.extend(iter::repeat(block).take(x));
        acc
    });

    DiskMap { blocks }
}

#[aoc(day9, part1)]
fn part1(disk: &DiskMap) -> usize {
    let mut disk = disk.clone();
    disk.defrag();
    disk.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        let exp = String::from("00...111...2...333.44.5555.6666.777.888899");
        assert_eq!(input.to_string(), exp);
    }

    #[test]
    fn test_part_one() {
        let input = input_generator(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 1928);
    }
}
