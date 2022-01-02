use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("L{0}")]
    Left(usize),

    #[display("R{0}")]
    Right(usize),
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Vec2d {
    x: i64,
    y: i64,
}

impl Vec2d {
    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Copy, Clone)]
struct State {
    dir: Direction,
    pos: Vec2d,
}

impl State {
    fn new() -> Self {
        Self {
            dir: Direction::North,
            pos: Vec2d { x: 0, y: 0 },
        }
    }

    fn handle_inst(&mut self, inst: Instruction) -> usize {
        match inst {
            Instruction::Left(n) => {
                self.turn_left();
                n
            }
            Instruction::Right(n) => {
                self.turn_right();
                n
            }
        }
    }

    fn turn_left(&mut self) {
        self.dir = self.dir.turn_left();
    }

    fn turn_right(&mut self) {
        self.dir = self.dir.turn_right();
    }
}

#[aoc_generator(day01)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.split(", ")
        .filter_map(|it| it.parse().ok())
        .collect_vec()
}

#[aoc(day01, part1)]
pub fn part1(inp: &[Instruction]) -> i64 {
    let state = inp.iter().fold(State::new(), |mut state, it| {
        let n = state.handle_inst(*it);
        let dir = get_direction_offset(state.dir);
        for _ in 0..n {
            state.pos.x += dir.x;
            state.pos.y += dir.y;
        }

        state
    });

    state.pos.manhattan_distance()
}

fn get_direction_offset(dir: Direction) -> Vec2d {
    match dir {
        Direction::North => Vec2d { x: 0, y: 1 },
        Direction::East => Vec2d { x: 1, y: 0 },
        Direction::South => Vec2d { x: 0, y: -1 },
        Direction::West => Vec2d { x: -1, y: 0 },
    }
}

fn try_walk(
    state: &mut State,
    steps: usize,
    dir: Vec2d,
    visited: &mut HashSet<Vec2d>,
) -> Option<Vec2d> {
    for _ in 0..steps {
        state.pos.x += dir.x;
        state.pos.y += dir.y;

        if visited.contains(&state.pos) {
            return Some(state.pos);
        }

        visited.insert(state.pos);
    }

    None
}

#[aoc(day01, part2)]
pub fn part2(inp: &[Instruction]) -> i64 {
    let mut state = State::new();
    let mut visited = HashSet::new();
    visited.insert(Vec2d { x: 0, y: 0 });

    for it in inp {
        let n = state.handle_inst(*it);
        let dir = get_direction_offset(state.dir);
        if let Some(pos) = try_walk(&mut state, n, dir, &mut visited) {
            return pos.manhattan_distance();
        }
    }

    unreachable!()
}
