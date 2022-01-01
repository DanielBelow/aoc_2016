use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day01)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect_vec()
}

#[aoc(day01, part1)]
pub fn part1(inp: &[String]) -> u64 {
    0
}

#[aoc(day01, part2)]
pub fn part2(inp: &[String]) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
}
