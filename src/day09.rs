use aoc_runner_derive::aoc;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("({num_chars}x{times})")]
struct Repeat {
    num_chars: usize,
    times: usize,
}

fn extract_repeat(s: &mut String) -> Option<Repeat> {
    if !s.starts_with('(') {
        return None;
    }

    let end = s.find(')')?;
    s.drain(..=end).as_str().parse::<Repeat>().ok()
}

fn decompress(s: &mut String, p1: bool) -> usize {
    let mut result = 0;

    while !s.is_empty() {
        if let Some(repeat) = extract_repeat(s) {
            let mut substr = s.drain(..repeat.num_chars).collect::<String>();
            if p1 {
                result += repeat.num_chars * repeat.times;
            } else {
                result += repeat.times * decompress(&mut substr, p1);
            }
        } else {
            result += 1;
            s.drain(..1);
        }
    }

    result
}

#[aoc(day09, part1)]
pub fn part1(inp: &str) -> usize {
    let mut inp = inp.to_string();
    decompress(&mut inp, true)
}

#[aoc(day09, part2)]
pub fn part2(inp: &str) -> usize {
    let mut inp = inp.to_string();
    decompress(&mut inp, false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let res = part1("ADVENT");
        assert_eq!(res, 6);

        let res = part1("A(1x5)BC");
        assert_eq!(res, 7);

        let res = part1("(3x3)XYZ");
        assert_eq!(res, 9);

        let res = part1("A(2x2)BCD(2x2)EFG");
        assert_eq!(res, 11);

        let res = part1("(6x1)(1x3)A");
        assert_eq!(res, 6);

        let res = part1("X(8x2)(3x3)ABCY");
        assert_eq!(res, 18);
    }

    #[test]
    fn test_sample_p2() {
        let res = part2("(3x3)XYZ");
        assert_eq!(res, 9);

        let res = part2("X(8x2)(3x3)ABCY");
        assert_eq!(res, 20);

        let res = part2("(27x12)(20x12)(13x14)(7x10)(1x12)A");
        assert_eq!(res, 241_920);

        let res = part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");
        assert_eq!(res, 445);
    }
}
