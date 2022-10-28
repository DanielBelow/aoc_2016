use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::FromStr;

#[derive(FromStr, Debug, Copy, Clone)]
#[from_str(
    regex = "Disc #[0-9]+ has (?P<num_positions>[0-9]+) positions; at time=0, it is at position (?P<position>[0-9]+)."
)]
pub struct Disk {
    num_positions: usize,
    pub position: usize,
}

impl Disk {
    fn advance_by(&mut self, steps: usize) {
        self.position = (self.position + steps) % self.num_positions;
    }
}

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Vec<Disk> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn find_num_required_steps(disks: &[Disk]) -> usize {
    for t in 0.. {
        let mut disks = disks.to_vec();
        for d in &mut disks {
            d.advance_by(t);
        }

        let found_solution = disks
            .iter()
            .enumerate()
            .all(|(i, d)| (d.position + i + 1) % d.num_positions == 0);

        if found_solution {
            return t;
        }
    }

    unreachable!("No solution found?")
}

#[aoc(day15, part1)]
pub fn part1(inp: &[Disk]) -> usize {
    find_num_required_steps(inp)
}

#[aoc(day15, part2)]
pub fn part2(inp: &[Disk]) -> usize {
    let mut inp = inp.to_vec();
    inp.push(Disk {
        num_positions: 11,
        position: 0,
    });

    find_num_required_steps(&inp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_advance() {
        let mut disk = Disk {
            num_positions: 3,
            position: 2,
        };

        for i in 0..=2 {
            disk.advance_by(1);
            assert_eq!(i, disk.position);
        }
    }

    #[test]
    fn test_advance_by() {
        let mut disk = Disk {
            num_positions: 3,
            position: 2,
        };

        disk.advance_by(3);
        assert_eq!(2, disk.position);
    }

    #[test]
    fn test_sample_p1() {
        let disks = vec![
            Disk {
                num_positions: 5,
                position: 4,
            },
            Disk {
                num_positions: 2,
                position: 1,
            },
        ];

        let res = part1(&disks);
        assert_eq!(5, res);
    }
}
