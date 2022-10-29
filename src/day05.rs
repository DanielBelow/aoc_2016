use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day05, part1)]
pub fn part1(inp: &str) -> Option<String> {
    let mut password = String::new();

    for idx in 0usize.. {
        if password.len() == 8 {
            break;
        }

        let inp_with_idx = format!("{inp}{idx}");
        let digest = format!("{:x}", md5::compute(inp_with_idx));
        if digest.starts_with("00000") {
            let sixth_char = digest.chars().nth(5)?;
            password.push(sixth_char);
        }
    }

    Some(password)
}

#[aoc(day05, part2)]
pub fn part2(inp: &str) -> Option<String> {
    const PLACEHOLDER: char = '_';

    let mut password = vec![PLACEHOLDER; 8];
    let mut num_found = 0;

    for idx in 0usize.. {
        if num_found == 8 {
            return Some(password.iter().join(""));
        }

        let inp_with_idx = format!("{inp}{idx}");
        let digest = format!("{:x}", md5::compute(inp_with_idx));
        if digest.starts_with("00000") {
            let sixth_char = digest.chars().nth(5)?;
            if let Some(dgt) = sixth_char.to_digit(10) {
                if dgt < 8 {
                    let idx = usize::try_from(dgt).ok()?;
                    let seventh_char = digest.chars().nth(6)?;
                    if password[idx] == PLACEHOLDER {
                        password[idx] = seventh_char;
                        num_found += 1;
                    }
                }
            }
        }
    }

    unreachable!()
}
