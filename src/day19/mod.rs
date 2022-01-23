use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Option<usize> {
    inp.parse().ok()
}

#[aoc(day19, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part1(inp: &usize) -> Option<usize> {
    let mut elfs = vec![true; *inp];
    let num_elfs = elfs.len();

    let find_next_index = |mut start_idx: usize, elfs: &Vec<bool>| loop {
        if start_idx >= num_elfs {
            start_idx %= num_elfs;
        }

        if elfs[start_idx] {
            return start_idx;
        }

        start_idx += 1;
    };

    while elfs.iter().filter(|it| **it).count() != 1 {
        for idx in 0..num_elfs {
            if !elfs[idx] {
                continue;
            }

            let idx = find_next_index(idx + 1, &elfs);
            elfs[idx] = false;
        }
    }

    elfs.iter().position(|it| *it).map(|it| it + 1)
}

#[aoc(day19, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part2(_: &usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let res = part1(&5);
        assert_eq!(res.unwrap(), 3);
    }
}
