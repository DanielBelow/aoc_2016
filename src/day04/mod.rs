use aoc_runner_derive::{aoc, aoc_generator};
use cipher_crypt::{Caesar, Cipher};
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Room {
    name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        let checksum = self
            .name
            .chars()
            .filter(|it| *it != '-')
            .counts()
            .into_iter()
            .sorted_by(|(k, v), (kk, vv)| match v.cmp(vv).reverse() {
                std::cmp::Ordering::Equal => k.cmp(kk),
                other => other,
            })
            .take(5)
            .map(|(c, _)| c)
            .join("");

        checksum == self.checksum
    }

    fn shift(&self) -> Self {
        let cipher = Caesar::new(self.sector_id % 26);
        let decrypted = cipher.encrypt(&self.name).unwrap();

        Self {
            name: decrypted,
            sector_id: self.sector_id,
            checksum: self.checksum.clone(),
        }
    }
}

#[aoc_generator(day04)]
pub fn generate(inp: &str) -> Vec<Room> {
    inp.lines().fold(Vec::new(), |mut acc, it| {
        if let Some((name, rest)) = it.rsplit_once('-') {
            if let Some((sector_id, rest)) = rest.split_once('[') {
                let sector_id = sector_id.parse::<usize>().unwrap_or(0);
                let checksum = rest.chars().dropping_back(1).join("");
                acc.push(Room {
                    name: name.to_string(),
                    sector_id,
                    checksum,
                });
            }
        }

        acc
    })
}

#[aoc(day04, part1)]
pub fn part1(inp: &[Room]) -> usize {
    inp.iter()
        .filter(|it| it.is_real())
        .fold(0, |acc, it| acc + it.sector_id)
}

#[aoc(day04, part2)]
pub fn part2(inp: &[Room]) -> Option<usize> {
    inp.iter()
        .filter(|it| it.is_real())
        .map(Room::shift)
        .find(|it| it.name == "northpole-object-storage")
        .map(|it| it.sector_id)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "aaaaa-bbb-z-y-x-123[abxyz]\n\
                             a-b-c-d-e-f-g-h-987[abcde]\n\
                             not-a-real-room-404[oarel]\n\
                             totally-real-room-200[decoy]";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 1514);
    }
}
