use super::assembunny::{parse_instructions, Instruction};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    parse_instructions(inp)
}

#[aoc(day23, part1)]
pub fn part1(_: &[Instruction]) -> u64 {
    0
}

#[aoc(day23, part2)]
pub fn part2(_: &[Instruction]) -> u64 {
    0
}
