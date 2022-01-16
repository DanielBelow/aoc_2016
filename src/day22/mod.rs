use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Clone, Debug)]
#[from_str(
    regex = "(?P<filesystem>([^\\s]+))\\s+(?P<size>([0-9]+))T\\s+(?P<used>([0-9]+))T\\s+(?P<avail>([0-9]+))T\\s+(?P<use_percent>([0-9]+))%"
)]
pub struct NodeInfo {
    filesystem: String,
    size: usize,
    used: usize,
    avail: usize,
    use_percent: usize,
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Vec<NodeInfo> {
    inp.lines()
        .skip(2)
        .filter_map(|it| it.parse().ok())
        .collect()
}

#[aoc(day22, part1)]
pub fn part1(inp: &[NodeInfo]) -> u64 {
    let mut result = 0;

    let check_node_pair = |a: &NodeInfo, b: &NodeInfo| -> bool { a.used != 0 && a.used <= b.avail };

    for comb in inp.iter().combinations(2) {
        if let [a, b] = comb[..] {
            if check_node_pair(a, b) {
                result += 1;
            }

            if check_node_pair(b, a) {
                result += 1;
            }
        }
    }

    result
}

#[aoc(day22, part2)]
pub fn part2(_: &[NodeInfo]) -> u64 {
    0
}
