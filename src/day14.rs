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
        target.map_or_else(|| it.iter().all_equal(), |tgt| it.iter().all(|c| *c == tgt))
    });

    res.map(|it| it[0])
}

fn has_three_repetitions(hash: &str) -> Option<char> {
    has_repetitions(3, hash, None)
}

fn next_hash_has_five_repetitions(
    num_hashes: usize,
    idx: usize,
    target: char,
    cache: &mut HashCache<'_>,
) -> bool {
    (idx + 1..idx + num_hashes).any(|it| {
        let hash = cache.calculate_hash(it);
        has_repetitions(5, hash, Some(target)).is_some()
    })
}

fn stretch_key(num_stretches: usize, input: &str, idx: usize) -> String {
    let to_hash = format!("{input}{idx}");

    let hash = |txt: &str| -> String { format!("{:x}", md5::compute(txt)) };
    (0..num_stretches).fold(hash(&to_hash), |acc, _| hash(&acc))
}

fn is_key(idx: usize, cache: &mut HashCache<'_>) -> bool {
    let hash = cache.calculate_hash(idx);

    has_three_repetitions(hash).map_or(false, |it| {
        next_hash_has_five_repetitions(1_000, idx, it, cache)
    })
}

fn find_idx_for_nth_hash(n: usize, cache: &mut HashCache<'_>) -> usize {
    let mut seen_keys = 0;

    for cur_idx in 0.. {
        seen_keys += usize::from(is_key(cur_idx, cache));

        if seen_keys == n {
            return cur_idx;
        }
    }

    unreachable!()
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
