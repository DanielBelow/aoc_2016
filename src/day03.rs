use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Copy, Clone, Debug)]
#[from_str(regex = "\\s*(?P<a>[0-9]+)\\s*(?P<b>[0-9]+)\\s*(?P<c>[0-9]+)")]
pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    fn is_valid(&self) -> bool {
        if self.b + self.c <= self.a {
            return false;
        }

        if self.a + self.c <= self.b {
            return false;
        }

        if self.a + self.b <= self.c {
            return false;
        }

        true
    }
}

#[aoc_generator(day03, part1)]
pub fn generate_p1(inp: &str) -> Vec<Triangle> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc_generator(day03, part2)]
pub fn generate_p2(inp: &str) -> Vec<Triangle> {
    let mut res = Vec::new();

    for (t1, t2, t3) in inp
        .lines()
        .filter_map(|it| it.parse::<Triangle>().ok())
        .tuple_windows::<(_, _, _)>()
        .step_by(3)
    {
        res.push(Triangle::new(t1.a, t2.a, t3.a));
        res.push(Triangle::new(t1.b, t2.b, t3.b));
        res.push(Triangle::new(t1.c, t2.c, t3.c));
    }

    res
}

#[aoc(day03, part1)]
pub fn part1(inp: &[Triangle]) -> usize {
    inp.iter().filter(|it| it.is_valid()).count()
}

#[aoc(day03, part2)]
pub fn part2(inp: &[Triangle]) -> usize {
    inp.iter().filter(|it| it.is_valid()).count()
}
