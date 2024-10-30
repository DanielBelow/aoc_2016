use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day07)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect()
}

fn supports_tls(ip: &str) -> bool {
    let mut is_valid = false;
    let mut is_inside_brackets = false;

    for idx in 0..ip.len() - 3 {
        let cur_char = ip.chars().nth(idx).expect("Invalid idx");
        if cur_char == '[' {
            is_inside_brackets = true;
        } else if cur_char == ']' {
            is_inside_brackets = false;
        }

        let next_char = ip.chars().nth(idx + 1).expect("Invalid idx + 1");
        if next_char != cur_char
            && cur_char == ip.chars().nth(idx + 3).expect("Invalid idx + 3")
            && next_char == ip.chars().nth(idx + 2).expect("Invalid idx + 2")
        {
            if is_inside_brackets {
                return false;
            }

            is_valid = true;
        }
    }

    is_valid
}

fn supports_ssl(ip: &str) -> bool {
    let mut is_inside_brackets = false;

    let mut triples_inside = Vec::new();
    let mut triples_outside = Vec::new();

    for idx in 0..ip.len() - 2 {
        let cur_char = ip.chars().nth(idx).expect("Invalid idx");
        if cur_char == '[' {
            is_inside_brackets = true;
        } else if cur_char == ']' {
            is_inside_brackets = false;
        }

        let next_char = ip.chars().nth(idx + 1).expect("Invalid idx + 1");
        if next_char != cur_char && cur_char == ip.chars().nth(idx + 2).expect("Invalid idx + 2") {
            if is_inside_brackets {
                triples_inside.push((cur_char, next_char));
            } else {
                triples_outside.push((cur_char, next_char));
            }
        }
    }

    for (cur, next) in triples_outside {
        if triples_inside.contains(&(next, cur)) {
            return true;
        }
    }

    false
}

#[aoc(day07, part1)]
pub fn part1(inp: &[String]) -> usize {
    inp.iter()
        .fold(0, |acc, it| if supports_tls(it) { acc + 1 } else { acc })
}

#[aoc(day07, part2)]
pub fn part2(inp: &[String]) -> usize {
    inp.iter()
        .fold(0, |acc, it| if supports_ssl(it) { acc + 1 } else { acc })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_p1() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));

        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
    }

    #[test]
    fn test_sample_p2() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));

        assert!(!supports_ssl("xyx[xyx]xyx"));
    }
}
