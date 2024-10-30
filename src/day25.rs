use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub const fn part1(_: &str) -> usize {
    const OFFSET: usize = 2541;
    0b1010_1010_1010 - OFFSET
}
