use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day06)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect()
}

fn get_char_frequencies_at(idx: usize, inp: &[String]) -> HashMap<char, usize> {
    inp.iter().filter_map(|it| it.chars().nth(idx)).counts()
}

#[aoc(day06, part1)]
pub fn part1(inp: &[String]) -> Option<String> {
    let mut result = String::new();

    for idx in 0..8 {
        let freqs = get_char_frequencies_at(idx, inp);
        let (&c, _) = freqs.iter().max_by_key(|(_, &v)| v)?;
        result.push(c);
    }

    Some(result)
}

#[aoc(day06, part2)]
pub fn part2(inp: &[String]) -> Option<String> {
    let mut result = String::new();

    for idx in 0..8 {
        let freqs = get_char_frequencies_at(idx, inp);
        let (&c, _) = freqs.iter().min_by_key(|(_, &v)| v)?;
        result.push(c);
    }

    Some(result)
}
