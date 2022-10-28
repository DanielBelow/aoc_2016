use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::VecDeque;

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Option<usize> {
    inp.parse().ok()
}

#[aoc(day19, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part1(inp: &usize) -> usize {
    let binary = format!("{inp:b}");

    let mut vec = binary.chars().map(|it| it.to_string()).collect_vec();
    vec.rotate_left(1);

    let joined = vec.join("");
    usize::from_str_radix(&joined, 2).unwrap()
}

#[aoc(day19, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part2(inp: &usize) -> Option<usize> {
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();

    for i in 1..=*inp {
        if i < *inp / 2 + 1 {
            left.push_back(i);
        } else {
            right.push_front(i);
        }
    }

    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }

        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }

    left.get(0).or_else(|| right.get(0)).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let res = part1(&5);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_sample_p2() {
        let res = part2(&5);
        assert_eq!(res.unwrap(), 2);
    }
}
