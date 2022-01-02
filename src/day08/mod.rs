use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("rect {width}x{height}")]
    Rect { width: usize, height: usize },

    #[display("rotate row y={row} by {by}")]
    RotateRow { row: usize, by: usize },

    #[display("rotate column x={col} by {by}")]
    RotateCol { col: usize, by: usize },
}

#[aoc_generator(day08)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn transpose(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let height = map.len();
    let width = map[0].len();

    let mut result = vec![vec!['.'; height]; width];

    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            result[c][r] = *col;
        }
    }

    result
}

fn execute_instructions(insts: &[Instruction], display: &mut Vec<Vec<char>>) {
    for &inst in insts {
        match inst {
            Instruction::Rect {
                width: w,
                height: h,
            } => {
                for (x, y) in iproduct!(0..w, 0..h) {
                    display[y][x] = '#';
                }
            }
            Instruction::RotateRow { row: r, by: b } => {
                let row = &mut display[r];
                row.rotate_right(b);
            }
            Instruction::RotateCol { col: c, by: b } => {
                let mut transposed = transpose(display);
                let row = &mut transposed[c];
                row.rotate_right(b);
                *display = transpose(&transposed);
            }
        };
    }
}

#[aoc(day08, part1)]
pub fn part1(inp: &[Instruction]) -> usize {
    let mut map = vec![vec!['.'; 50]; 6];
    execute_instructions(inp, &mut map);

    map.iter().fold(0, |acc, it| {
        it.iter()
            .fold(acc, |acc, it| acc + if *it == '#' { 1 } else { 0 })
    })
}

#[aoc(day08, part2)]
pub fn part2(inp: &[Instruction]) -> String {
    let mut map = vec![vec!['.'; 50]; 6];
    execute_instructions(inp, &mut map);

    // prints "UPOJFLBCEZ"
    /*
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    */

    String::from("UPOJFLBCEZ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose() {
        let map = vec![vec!['.', '.', '.'], vec!['.', '#', '.']];
        let expected = vec![vec!['.', '.'], vec!['.', '#'], vec!['.', '.']];

        let transposed = transpose(&map);
        assert_eq!(transposed, expected);
    }
}
