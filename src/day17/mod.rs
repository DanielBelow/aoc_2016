use aoc_runner_derive::aoc;
use itertools::Itertools;
use pathfinding::prelude::{bfs, topological_sort};

const INPUT: &str = "mmsxrhfx";

fn hash(data: &str) -> String {
    let digest = md5::compute(data);
    let hash = format!("{digest:x}");
    hash.chars().take(4).join("")
}

fn get_successors(x: usize, y: usize, path: &str) -> Vec<(usize, usize, String)> {
    if x == 3 && y == 3 {
        return vec![];
    }

    let to_hash = format!("{INPUT}{path}");
    let cur_hash = hash(&to_hash);
    let directions = cur_hash
        .chars()
        .map(|it| matches!(it, 'b' | 'c' | 'd' | 'e' | 'f'))
        .collect_vec();

    let mut successors = vec![];

    // directions are UP, DOWN, LEFT, RIGHT
    if y > 0 && directions[0] {
        successors.push((x, y - 1, format!("{path}U")));
    }
    if y < 3 && directions[1] {
        successors.push((x, y + 1, format!("{path}D")));
    }

    if x > 0 && directions[2] {
        successors.push((x - 1, y, format!("{path}L")));
    }
    if x < 3 && directions[3] {
        successors.push((x + 1, y, format!("{path}R")));
    }

    successors
}

#[aoc(day17, part1)]
pub fn part1(_: &str) -> Option<String> {
    bfs(
        &(0usize, 0usize, String::new()),
        |(x, y, path)| get_successors(*x, *y, path),
        |(x, y, _)| *x == 3 && *y == 3,
    )
    .and_then(|it| it.last().map(|(_, _, p)| p).cloned())
}

#[aoc(day17, part2)]
pub fn part2(_: &str) -> Option<usize> {
    topological_sort(&[(0usize, 0usize, String::new())], |(x, y, path)| {
        get_successors(*x, *y, path)
    })
    .ok()
    .and_then(|it| {
        it.iter()
            .filter(|(x, y, _)| *x == 3 && *y == 3)
            .max_by_key(|(_, _, p)| p.len())
            .cloned()
    })
    .map(|(_, _, it)| it.len())
}
