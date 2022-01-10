use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{from}-{to}")]
pub struct Range {
    from: usize,
    to: usize,
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<Range> {
    inp.lines()
        .filter_map(|it| it.parse::<Range>().ok())
        .sorted_by_key(|it| it.from)
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(blacklist: &[Range]) -> Option<usize> {
    let mut valid_ips = vec![true; 4_294_967_296_usize];

    for range in blacklist {
        for e in &mut valid_ips[range.from..=range.to] {
            *e = false;
        }
    }

    valid_ips.iter().position(|it| *it)
}

#[aoc(day20, part2)]
pub fn part2(blacklist: &[Range]) -> usize {
    let mut valid_ips = vec![true; 4_294_967_296_usize];

    for range in blacklist {
        for e in &mut valid_ips[range.from..=range.to] {
            *e = false;
        }
    }

    valid_ips.iter().filter(|it| **it).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "5-8\n\
                             0-2\n\
                             4-7";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen).unwrap();
        assert_eq!(res, 3);
    }
}
