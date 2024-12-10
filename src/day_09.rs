use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Formatter, Write};
use std::ops::Range;
use std::{fmt, iter};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum FileBlock {
    FileId(usize),
    FreeSpace,
}

impl FileBlock {
    fn is_free_space(&self) -> bool {
        matches!(self, FileBlock::FreeSpace)
    }
}
impl fmt::Display for FileBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            FileBlock::FileId(id) => char::from_digit((*id % 36) as u32, 36).unwrap(),
            FileBlock::FreeSpace => '.',
        };

        f.write_char(c)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct FilePointer {
    id: usize,
    index: usize,
    block_size: usize,
}

impl FilePointer {
    fn range(&self) -> Range<usize> {
        Range {
            start: self.index,
            end: self.index + self.block_size,
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
struct DiskMap {
    blocks: Vec<FileBlock>,
    orig_file_order: Vec<FilePointer>,
}

impl fmt::Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.blocks.iter().try_for_each(|fb| fb.fmt(f))
    }
}

impl DiskMap {
    fn first_free(&self, start: usize) -> usize {
        let mut cursor = start;
        while self.blocks[cursor] != FileBlock::FreeSpace {
            cursor += 1;
        }
        cursor
    }

    fn defrag(&mut self) {
        //advance cursor to first empty block
        let mut cursor = self.first_free(0);

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

    fn find_empty_span(&self, size: usize, max_idx: usize) -> Option<Range<usize>> {
        //advance cursor to first empty block
        let first = self.first_free(0);
        let mut range = Range {
            start: first,
            end: first + size,
        };

        while range.end <= max_idx
            && self.blocks[range.clone()]
                .iter()
                .any(|b| !b.is_free_space())
        {
            range.start = self.first_free(range.start + 1);
            range.end = range.start + size;
        }

        if range.end <= max_idx {
            Some(range)
        } else {
            None
        }
    }

    fn mv_file(&mut self, file: &FilePointer) {
        if let Some(empty_range) = self.find_empty_span(file.block_size, file.index) {
            file.range()
                .zip(empty_range)
                .for_each(|(file_block_idx, empty_block_idx)| {
                    self.blocks.swap(file_block_idx, empty_block_idx);
                });
        }
    }

    fn mv_files(&mut self) {
        self.orig_file_order
            .clone()
            .iter()
            .rev()
            .for_each(|file_pointer| {
                self.mv_file(file_pointer);
            });
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                FileBlock::FileId(fid) => i * *fid,
                FileBlock::FreeSpace => 0,
            })
            .sum()
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> DiskMap {
    let mut files = vec![];
    let blocks: Vec<FileBlock> = input.chars().enumerate().fold(vec![], |mut acc, (i, c)| {
        let x = c.to_string().parse().unwrap();

        let block = if i % 2 == 0 {
            // even index is a file id
            let file_id = i / 2;

            //record in files
            files.push(FilePointer {
                id: file_id,
                index: acc.len(),
                block_size: x,
            });

            FileBlock::FileId(file_id)
        } else {
            FileBlock::FreeSpace
        };

        acc.extend(iter::repeat(block).take(x));
        acc
    });

    DiskMap {
        blocks,
        orig_file_order: files,
    }
}

#[aoc(day9, part1)]
fn part1(disk: &DiskMap) -> usize {
    let mut disk = disk.clone();
    disk.defrag();
    disk.checksum()
}

#[aoc(day9, part2)]
fn part2(disk: &DiskMap) -> usize {
    let mut disk = disk.clone();
    disk.mv_files();
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

    #[test]
    fn test_part_two() {
        let input = input_generator(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 2858);
    }
}
