use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[from_str(
    regex = "/dev/grid/node-x(?P<x>([0-9]+))-y(?P<y>([0-9]+))\\s+(?P<size>([0-9]+))T\\s+(?P<used>([0-9]+))T\\s+(?P<avail>([0-9]+))T\\s+(?P<use_percent>([0-9]+))%"
)]
pub struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    use_percent: usize,
    #[from_str(default)]
    is_target: bool,
}

impl Node {
    fn is_empty(&self) -> bool {
        self.used == 0
    }

    fn fits_onto(&self, other: &Node) -> bool {
        self.used <= (other.size - other.used)
    }
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Vec<Vec<Node>> {
    let mut nodes = vec![];

    for line in inp.lines().skip(2) {
        let node = line.parse::<Node>().expect("Couldn't parse node");
        if node.x == 0 {
            nodes.push(vec![node]);
        } else {
            nodes[node.y].push(node);
        }
    }

    let target_node = nodes
        .iter_mut()
        .flatten()
        .filter(|it| it.y == 0)
        .max_by_key(|it| it.x)
        .unwrap();
    target_node.is_target = true;

    nodes
}

fn part1_impl(nodes: &[Vec<Node>]) -> Vec<(Node, Node)> {
    let check_node_pair = |a: &Node, b: &Node| -> bool { !a.is_empty() && a.fits_onto(b) };

    nodes
        .iter()
        .flatten()
        .combinations(2)
        .inspect(|it| assert_eq!(it.len(), 2))
        .map(|it| (*it[0], *it[1]))
        .filter(|(lhs, rhs)| check_node_pair(lhs, rhs) || check_node_pair(rhs, lhs))
        .collect()
}

#[aoc(day22, part1)]
pub fn part1(nodes: &[Vec<Node>]) -> usize {
    part1_impl(nodes).len()
}

#[aoc(day22, part2)]
pub fn part2(_: &[Vec<Node>]) -> usize {
    /*
    let available_pairs = part1_impl(nodes)
        .iter()
        .flat_map(|(lhs, rhs)| [*lhs, *rhs])
        .collect::<HashSet<_>>();

    for row in nodes {
        for node in row {
            if node.is_empty() {
                print!(" _ ");
            } else if node.is_target {
                print!(" G ");
            } else if !available_pairs.contains(node) {
                print!(" # ");
            } else {
                print!(" . ");
            }
        }

        println!();
    }

    println!();
    */

    /*
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  G
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  #  #  #  #  #  #  #  #  #  #  #  #  #  #
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  _  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
    */

    // 34 moves to have '_' left of 'G'
    // 1 move to swap the two
    // 33 times 5 moves to swap 'G' to the left-most node

    34 + 1 + 5 * 33
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "root@ebhq-gridcenter# df -h\n\
                             Filesystem            Size  Used  Avail  Use%\n\
                             /dev/grid/node-x0-y0   10T    8T     2T   80%\n\
                             /dev/grid/node-x0-y1   11T    6T     5T   54%\n\
                             /dev/grid/node-x0-y2   32T   28T     4T   87%\n\
                             /dev/grid/node-x1-y0    9T    7T     2T   77%\n\
                             /dev/grid/node-x1-y1    8T    0T     8T    0%\n\
                             /dev/grid/node-x1-y2   11T    7T     4T   63%\n\
                             /dev/grid/node-x2-y0   10T    6T     4T   60%\n\
                             /dev/grid/node-x2-y1    9T    8T     1T   88%\n\
                             /dev/grid/node-x2-y2    9T    6T     3T   66%";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let result = part1(&data);
        assert_eq!(7, result);
    }
}
