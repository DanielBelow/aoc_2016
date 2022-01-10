use aoc_runner_derive::aoc;
use itertools::{iproduct, Itertools};

const GRID_SIZE: usize = 100;

fn get_successors(x: usize, y: usize, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut successors = Vec::new();

    if y > 0 && grid[y - 1][x] == '.' {
        successors.push((x, y - 1));
    }
    if x > 0 && grid[y][x - 1] == '.' {
        successors.push((x - 1, y));
    }
    if y < GRID_SIZE - 1 && grid[y + 1][x] == '.' {
        successors.push((x, y + 1));
    }
    if x < GRID_SIZE - 1 && grid[y][x + 1] == '.' {
        successors.push((x + 1, y));
    }

    successors
}

fn setup_grid(inp: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

    for (x, y) in iproduct!(0..GRID_SIZE, 0..GRID_SIZE) {
        let r = inp + (x * x + 3 * x + 2 * x * y + y + y * y);
        let ones = r.count_ones();
        grid[y][x] = if ones % 2 == 0 { '.' } else { '#' };
    }

    grid
}

#[aoc(day13, part1)]
pub fn part1(inp: &str) -> usize {
    let inp = inp.parse::<usize>().unwrap();
    let grid = setup_grid(inp);

    pathfinding::prelude::bfs(
        &(1usize, 1usize),
        |&(x, y)| get_successors(x, y, &grid),
        |&(x, y)| x == 31 && y == 39,
    )
    .unwrap()
    .len()
}

#[aoc(day13, part2)]
pub fn part2(inp: &str) -> usize {
    let inp = inp.parse::<usize>().unwrap();

    let grid = setup_grid(inp);

    1 + pathfinding::prelude::dijkstra_all(&(1, 1), |&(x, y)| {
        let successors = get_successors(x, y, &grid);
        successors.iter().map(|it| (*it, 1)).collect_vec()
    })
    .iter()
    .filter(|(_, &(_, cost))| cost <= 50)
    .count()
}
