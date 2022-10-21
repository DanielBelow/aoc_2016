use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<String> {
    vec![inp.to_string()]
}

fn get_new_tile(left: char, center: char, right: char) -> char {
    match (left, center, right) {
        ('^', '^' | '.', '.') | ('.', '^' | '.', '^') => '^',
        _ => '.',
    }
}

fn get_next_row(current: &String) -> (String, usize) {
    assert!(!current.is_empty());
    let row_len = current.len();

    let mut num_safe_tiles = 0;

    let mut next_row = String::new();

    for i in 0..row_len {
        let left = if i > 0 {
            current.chars().nth(i - 1)
        } else {
            None
        };
        let left = left.unwrap_or('.');

        let center = current.chars().nth(i).expect("Valid index");

        let right = if i < row_len - 1 {
            current.chars().nth(i + 1)
        } else {
            None
        };
        let right = right.unwrap_or('.');

        let new_tile = get_new_tile(left, center, right);
        next_row.push(new_tile);

        num_safe_tiles += usize::from(new_tile == '.');
    }

    (next_row, num_safe_tiles)
}

fn run_for(inp: &[String], steps: usize) -> usize {
    let mut current = inp.last().expect("Input shouldn't be empty").clone();

    let mut result = current.chars().filter(|it| *it == '.').count();

    for _ in 0..steps {
        let (next, amount) = get_next_row(&current);
        current = next;
        result += amount;
    }

    result
}

#[aoc(day18, part1)]
pub fn part1(inp: &[String]) -> usize {
    run_for(inp, 40)
}

#[aoc(day18, part2)]
pub fn part2(inp: &[String]) -> usize {
    run_for(inp, 400_000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let (next_row, _) = get_next_row(&String::from("..^^."));
        assert_eq!(next_row, String::from(".^^^^"));
    }

    #[test]
    fn test_sample_2() {
        let (next_row, _) = get_next_row(&String::from(".^^^^"));
        assert_eq!(next_row, String::from("^^..^"));
    }
}
