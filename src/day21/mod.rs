use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum ScrambleInst {
    #[display("swap position {0} with position {1}")]
    SwapPosition(usize, usize),

    #[display("swap letter {0} with letter {1}")]
    SwapLetter(char, char),

    #[display("rotate left {0} steps")]
    RotateLeft(usize),

    #[display("rotate right {0} steps")]
    RotateRight(usize),

    #[display("rotate based on position of letter {0}")]
    RotatePosition(char),

    #[display("reverse positions {0} through {1}")]
    ReversePositions(usize, usize),

    #[display("move position {0} to position {1}")]
    MovePosition(usize, usize),
}

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Vec<ScrambleInst> {
    inp.lines()
        .map(|it| {
            // rotate right 1 step -> rotate right 1 steps
            if it.ends_with("step") {
                it.replace("step", "steps")
            } else {
                it.to_string()
            }
        })
        .filter_map(|it| it.parse().ok())
        .collect()
}

fn scramble(inp: &str, insts: &[ScrambleInst]) -> Option<String> {
    let mut inp = inp.chars().collect_vec();

    for inst in insts {
        match *inst {
            ScrambleInst::SwapPosition(x, y) => inp.swap(x, y),
            ScrambleInst::SwapLetter(a, b) => {
                let a_pos = inp.iter().position(|it| *it == a)?;
                let b_pos = inp.iter().position(|it| *it == b)?;
                inp.swap(a_pos, b_pos);
            }
            ScrambleInst::RotateLeft(n) => inp.rotate_left(n),
            ScrambleInst::RotateRight(n) => inp.rotate_right(n),
            ScrambleInst::RotatePosition(a) => {
                let index = inp.iter().position(|it| *it == a)?;
                inp.rotate_right(1);
                inp.rotate_right(index);
                if index >= 4 {
                    inp.rotate_right(1);
                }
            }
            ScrambleInst::ReversePositions(x, y) => inp[x..=y].reverse(),
            ScrambleInst::MovePosition(x, y) => {
                let chr = inp.remove(x);
                inp.insert(y, chr);
            }
        };
    }

    Some(inp.iter().join(""))
}

fn unscramble(inp: &str, insts: &[ScrambleInst]) -> Option<String> {
    let chars = inp.chars().collect_vec();
    for perm in chars.iter().copied().permutations(inp.len()) {
        let perm = perm.iter().join("");
        if scramble(&perm, insts).map_or(false, |it| it == inp) {
            return Some(perm);
        }
    }

    None
}

#[aoc(day21, part1)]
pub fn part1(inp: &[ScrambleInst]) -> Option<String> {
    scramble("abcdefgh", inp)
}

#[aoc(day21, part2)]
pub fn part2(inp: &[ScrambleInst]) -> Option<String> {
    let target = "fbgdceah";
    unscramble(target, inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ScrambleInst::{
        MovePosition, ReversePositions, RotateLeft, RotatePosition, SwapLetter, SwapPosition,
    };

    #[test]
    fn test_sample_p1() {
        let insts = vec![
            SwapPosition(4, 0),
            SwapLetter('d', 'b'),
            ReversePositions(0, 4),
            RotateLeft(1),
            MovePosition(1, 4),
            MovePosition(3, 0),
            RotatePosition('b'),
            RotatePosition('d'),
        ];

        let res = scramble("abcde", &insts).unwrap();
        assert_eq!(res, "decab");
    }
}
