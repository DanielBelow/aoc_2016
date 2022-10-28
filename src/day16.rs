use aoc_runner_derive::aoc;
use itertools::Itertools;

fn step(a: &str) -> String {
    let b = a
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("Invalid character"),
        })
        .join("");

    String::from(a) + "0" + b.as_str()
}

fn checksum(inp: &str) -> String {
    inp.chars()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .join("")
}

fn calculate_checksum(inp: &str, disk_size: usize) -> String {
    let mut cur_str = inp.to_string();

    while cur_str.len() < disk_size {
        cur_str = step(&cur_str);
    }

    let mut cs = cur_str.chars().take(disk_size).join("");
    cs = checksum(&cs);
    while cs.len() % 2 == 0 {
        cs = checksum(&cs);
    }

    cs
}

#[aoc(day16, part1)]
pub fn part1(inp: &str) -> String {
    calculate_checksum(inp, 272)
}

#[aoc(day16, part2)]
pub fn part2(inp: &str) -> String {
    calculate_checksum(inp, 35_651_584)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_p1() {
        assert_eq!(step("1"), "100");
        assert_eq!(step("0"), "001");
        assert_eq!(step("11111"), "11111000000");
    }
}
