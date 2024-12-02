# Advent of Code 2024

Solutions to Advent of Code 2024 in Rust.

## Setup

start with `just credentials <aoc_token>` to set up the aoc cli and register your auth token.

This is using the Cargo Aoc helper [gobanos/cargo-aoc](https://github.com/gobanos/cargo-aoc) and required a project
setup in [lib.rs](src/lib.rs) to set the correct year. Then it's just a matter of tagging the generator functions with
`#[aoc_generator(dayX)]` and the solver functions with `#[aoc(dayX, part1)]` (and part2.)

## Useful commands

The daily inputs can be grabbed by `just fetch <day>`. If `<day>` is omitted, it grabs the current day. The inputs are
stored in a private submodule at the request of the AoC maintainer.

The default `just` command lints and runs today's solution. If you'd like to run an older day, specify with `just run <day>`
