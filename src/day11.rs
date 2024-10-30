use aoc_runner_derive::{aoc, aoc_generator};
use std::hash::{Hash, Hasher};
use Item::{Chip, Generator};
use Kind::{Dilithium, Elerium, Plutonium, Promethium, Ruthenium, Strontium, Thulium};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Kind {
    Elerium,
    Dilithium,
    Plutonium,
    Promethium,
    Ruthenium,
    Strontium,
    Thulium,
}

impl Kind {
    const fn values() -> [Self; 7] {
        [
            Elerium, Dilithium, Plutonium, Promethium, Ruthenium, Strontium, Thulium,
        ]
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Item {
    Generator(Kind),
    Chip(Kind),
}

#[derive(Eq, Clone)]
pub struct State {
    elevator: usize,
    floors: Vec<Vec<Item>>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator.eq(&other.elevator) && self.find_pairs().eq(&other.find_pairs())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator.hash(state);
        self.find_pairs().hash(state);
    }
}

impl State {
    fn find_pairs(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for kind in Kind::values() {
            if let Some(gen_idx) = self
                .floors
                .iter()
                .position(|it| it.contains(&Generator(kind)))
            {
                let chip_idx = self
                    .floors
                    .iter()
                    .position(|it| it.contains(&Chip(kind)))
                    .expect("Generator without a chip!");

                result.push((gen_idx, chip_idx));
            }
        }

        result.sort_by_key(|(lhs, _)| *lhs);
        result
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|it| is_valid_floor(it))
    }

    const fn can_move_in_direction(&self, dir: Direction) -> bool {
        match dir {
            Direction::Up => self.elevator < 3,
            Direction::Down => self.elevator > 0,
        }
    }

    fn move_single_item(&self, idx: usize, dir: Direction) -> Option<Self> {
        if !self.can_move_in_direction(dir) {
            return None;
        }

        let target_floor = match dir {
            Direction::Up => self.elevator + 1,
            Direction::Down => self.elevator - 1,
        };

        let mut new_state = self.clone();

        let item = new_state.floors[self.elevator].remove(idx);
        new_state.floors[target_floor].push(item);

        new_state.elevator = target_floor;

        if new_state.is_valid() {
            Some(new_state)
        } else {
            None
        }
    }

    fn move_two_items(&self, i: usize, j: usize, dir: Direction) -> Option<Self> {
        if !self.can_move_in_direction(dir) {
            return None;
        }

        let target_floor = match dir {
            Direction::Up => self.elevator + 1,
            Direction::Down => self.elevator - 1,
        };

        let mut new_state = self.clone();

        let item1 = new_state.floors[self.elevator].remove(j);
        new_state.floors[target_floor].push(item1);

        let item2 = new_state.floors[self.elevator].remove(i);
        new_state.floors[target_floor].push(item2);

        new_state.elevator = target_floor;

        if new_state.is_valid() {
            Some(new_state)
        } else {
            None
        }
    }

    fn reachable_states(&self) -> Vec<Self> {
        let mut reachable_states = vec![];

        let items_on_current_floor = self.floors[self.elevator].len();

        for i in 0..items_on_current_floor {
            let mut moved_two_items_up = false;
            let mut moved_one_item_down = false;

            // Check if we can move a single item down
            if let Some(s) = self.move_single_item(i, Direction::Down) {
                reachable_states.push(s);
                moved_one_item_down = true;
            }

            for j in i + 1..items_on_current_floor {
                // Check if we can move two items up
                if let Some(s) = self.move_two_items(i, j, Direction::Up) {
                    reachable_states.push(s);
                    moved_two_items_up = true;
                }

                // If we can move a *single* item down, don't try to move two
                if !moved_one_item_down {
                    if let Some(s) = self.move_two_items(i, j, Direction::Down) {
                        reachable_states.push(s);
                        moved_one_item_down = true;
                    }
                }
            }

            // If we moved *two* items up, don't try to move a single one
            if !moved_two_items_up {
                if let Some(s) = self.move_single_item(i, Direction::Up) {
                    reachable_states.push(s);
                }
            }
        }

        reachable_states
    }

    fn done(&self) -> bool {
        self.floors[0].is_empty()
            && self.floors[1].is_empty()
            && self.floors[2].is_empty()
            && self.elevator == 3
    }
}

fn is_valid_floor(items: &[Item]) -> bool {
    let (chips, gens): (Vec<_>, Vec<_>) =
        items.iter().copied().partition(|it| matches!(it, Chip(_)));

    // We either need no generators at all, or each chip has to have a matching generator
    gens.is_empty()
        || chips.iter().all(|it| match it {
            Generator(_) => unreachable!("Should only contain chips!"),
            Chip(chip_kind) => gens.contains(&Generator(*chip_kind)),
        })
}

#[aoc_generator(day11, part1)]
pub fn generate1(_: &str) -> State {
    let floor1 = vec![
        Generator(Thulium),
        Chip(Thulium),
        Generator(Plutonium),
        Generator(Strontium),
    ];
    let floor2 = vec![Chip(Plutonium), Chip(Strontium)];
    let floor3 = vec![
        Generator(Promethium),
        Chip(Promethium),
        Generator(Ruthenium),
        Chip(Ruthenium),
    ];
    let floor4 = vec![];

    State {
        elevator: 0,
        floors: vec![floor1, floor2, floor3, floor4],
    }
}

#[aoc_generator(day11, part2)]
pub fn generate2(_: &str) -> State {
    let floor1 = vec![
        Generator(Elerium),
        Chip(Elerium),
        Generator(Dilithium),
        Chip(Dilithium),
        Generator(Thulium),
        Chip(Thulium),
        Generator(Plutonium),
        Generator(Strontium),
    ];
    let floor2 = vec![Chip(Plutonium), Chip(Strontium)];
    let floor3 = vec![
        Generator(Promethium),
        Chip(Promethium),
        Generator(Ruthenium),
        Chip(Ruthenium),
    ];
    let floor4 = vec![];

    State {
        elevator: 0,
        floors: vec![floor1, floor2, floor3, floor4],
    }
}

#[aoc(day11, part1)]
pub fn part1(state: &State) -> Option<usize> {
    assert!(state.is_valid());

    let result = pathfinding::prelude::astar(
        state,
        |it| it.reachable_states().into_iter().map(|it| (it, 1)),
        |it| {
            it.floors[3].len()
                + 2 * it.floors[2].len()
                + 3 * it.floors[1].len()
                + 4 * it.floors[0].len()
        },
        State::done,
    );

    result.map(|(_, cost)| cost)
}

#[aoc(day11, part2)]
pub fn part2(state: &State) -> Option<usize> {
    part1(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let state = State {
            elevator: 0,
            floors: vec![
                vec![Chip(Thulium), Chip(Strontium)],
                vec![Generator(Thulium)],
                vec![Generator(Strontium)],
                vec![],
            ],
        };

        let result = part1(&state);
        assert_eq!(result, Some(11));
    }
}
