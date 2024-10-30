use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

use num_complex::Complex;

#[derive(PartialEq, Eq, Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("L{0}")]
    Left(usize),

    #[display("R{0}")]
    Right(usize),
}

fn manhattan_distance(compl: Complex<i64>) -> i64 {
    compl.l1_norm()
}

#[derive(Copy, Clone)]
struct State {
    dir: Complex<i64>,
    pos: Complex<i64>,
}

impl State {
    const CCW_ROTATION: Complex<i64> = Complex::new(0, 1);
    const CW_ROTATION: Complex<i64> = Complex::new(0, -1);

    const fn new() -> Self {
        Self {
            dir: Complex::new(0, 1),
            pos: Complex::new(0, 0),
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
        self.dir *= Self::CCW_ROTATION;
    }

    fn turn_right(&mut self) {
        self.dir *= Self::CW_ROTATION;
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
        for _ in 0..n {
            state.pos += state.dir;
        }

        state
    });

    manhattan_distance(state.pos)
}

fn try_walk(
    state: &mut State,
    steps: usize,
    visited: &mut HashSet<Complex<i64>>,
) -> Option<Complex<i64>> {
    for _ in 0..steps {
        state.pos += state.dir;

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
    visited.insert(Complex::new(0, 0));

    for it in inp {
        let n = state.handle_inst(*it);
        if let Some(pos) = try_walk(&mut state, n, &mut visited) {
            return manhattan_distance(pos);
        }
    }

    unreachable!()
}
