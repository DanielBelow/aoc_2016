use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

struct HashCache<'a> {
    input: &'a str,
    stretching: usize,
    hashes: HashMap<usize, String>,
}

impl<'a> HashCache<'a> {
    fn new(input: &'a str, stretching: usize) -> Self {
        Self {
            input,
            stretching,
            hashes: HashMap::new(),
        }
    }

    fn calculate_hash(&mut self, idx: usize) -> &String {
        self.hashes
            .entry(idx)
            .or_insert_with(|| stretch_key(self.stretching, self.input, idx))
    }
}

fn has_repetitions(num: usize, hash: &str, target: Option<char>) -> Option<char> {
    let v = hash.chars().collect_vec();
    let res = v.windows(num).find(|it| {
        if let Some(tgt) = target {
            it.iter().all(|c| *c == tgt)
        } else {
            it.iter().all_equal()
        }
    });

    res.map(|it| it[0])
}

fn has_three_repetitions(hash: &str) -> Option<char> {
    has_repetitions(3, hash, None)
}

fn next_hash_has_five_repetitions(
    num_hashes: usize,
    mut idx: usize,
    target: Option<char>,
    cache: &mut HashCache<'_>,
) -> bool {
    for _ in 0..num_hashes {
        idx += 1;
        let hash = cache.calculate_hash(idx);
        if has_repetitions(5, hash, target).is_some() {
            return true;
        }
    }

    false
}

fn stretch_key(num_stretches: usize, input: &str, idx: usize) -> String {
    let to_hash = format!("{}{}", input, idx);

    let mut hash = format!("{:x}", md5::compute(&to_hash));
    for _ in 0..num_stretches {
        hash = format!("{:x}", md5::compute(&hash));
    }

    hash
}

fn is_key(idx: usize, cache: &mut HashCache<'_>) -> bool {
    let hash = cache.calculate_hash(idx);
    match has_three_repetitions(hash) {
        None => false,
        c => next_hash_has_five_repetitions(1000, idx, c, cache),
    }
}

fn find_idx_for_nth_hash(n: usize, cache: &mut HashCache<'_>) -> usize {
    let mut cur_idx = 0;
    let mut seen_keys = 0;
    loop {
        if is_key(cur_idx, cache) {
            seen_keys += 1;
        }

        if seen_keys == n {
            return cur_idx;
        }

        cur_idx += 1;
    }
}

#[aoc(day14, part1)]
pub fn part1(inp: &str) -> usize {
    let mut cache = HashCache::new(inp, 0);
    find_idx_for_nth_hash(64, &mut cache)
}

#[aoc(day14, part2)]
pub fn part2(inp: &str) -> usize {
    let mut cache = HashCache::new(inp, 2016);
    find_idx_for_nth_hash(64, &mut cache)
}
