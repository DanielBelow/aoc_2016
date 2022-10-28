use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day02)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect()
}

fn to_digit(moves: &str, map: &[Vec<char>], cur_pos: &mut (usize, usize)) -> char {
    let height = map.len();
    let width = map[0].len();

    for m in moves.chars() {
        match m {
            'U' => {
                let new_y = cur_pos.1.saturating_sub(1);
                if map[new_y][cur_pos.0] != '.' {
                    cur_pos.1 = new_y;
                }
            }
            'D' => {
                let new_y = cur_pos.1 + 1;
                if new_y < height && map[new_y][cur_pos.0] != '.' {
                    cur_pos.1 = new_y;
                }
            }
            'L' => {
                let new_x = cur_pos.0.saturating_sub(1);
                if map[cur_pos.1][new_x] != '.' {
                    cur_pos.0 = new_x;
                }
            }
            'R' => {
                let new_x = cur_pos.0 + 1;
                if new_x < width && map[cur_pos.1][new_x] != '.' {
                    cur_pos.0 = new_x;
                }
            }
            _ => unreachable!(),
        };
    }

    map[cur_pos.1][cur_pos.0]
}

#[aoc(day02, part1)]
pub fn part1(inp: &[String]) -> String {
    let map = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let mut cur_pos = (1, 1);
    inp.iter()
        .map(|it| to_digit(it, &map, &mut cur_pos))
        .join("")
}

#[aoc(day02, part2)]
pub fn part2(inp: &[String]) -> String {
    let map = vec![
        vec!['.', '.', '1', '.', '.'],
        vec!['.', '2', '3', '4', '.'],
        vec!['5', '6', '7', '8', '9'],
        vec!['.', 'A', 'B', 'C', '.'],
        vec!['.', '.', 'D', '.', '.'],
    ];

    let mut cur_pos = (0, 2);
    inp.iter()
        .map(|it| to_digit(it, &map, &mut cur_pos))
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_DATA: &str = "ULL\n\
                             RRDDD\n\
                             LURDL\n\
                             UUUUD";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, "1985");
    }

    #[test]
    fn test_sample_p2() {
        let gen = generate(TEST_DATA);
        let res = part2(&gen);
        assert_eq!(res, "5DB3");
    }
}
