use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Eq, Clone, Debug)]
pub enum Instruction {
    #[display("value {value} goes to {target}")]
    Input { value: i64, target: String },

    #[display("{bot} gives low to {low_target} and high to {high_target}")]
    Transfer {
        bot: String,
        low_target: String,
        high_target: String,
    },
}

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

fn find_transfer_instr_for_bot<'a>(
    to_find: &str,
    instructions: &'a [Instruction],
) -> Option<&'a Instruction> {
    instructions.iter().find(|it| match it {
        Instruction::Input { .. } => false,
        Instruction::Transfer { bot, .. } => *bot == to_find,
    })
}

fn run_instructions_until<T>(
    instructions: &[Instruction],
    bots: &mut HashMap<String, Vec<i64>>,
    pred: T,
) -> Option<usize>
where
    T: Fn(&Vec<i64>) -> bool,
{
    let mut queue = bots
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(k, _)| k.clone())
        .collect_vec();

    while !queue.is_empty() {
        let cur_bot = queue.pop().unwrap();

        let bot_values = bots.get_mut(&cur_bot)?;
        if pred(bot_values) {
            return cur_bot.strip_prefix("bot ").and_then(|it| it.parse().ok());
        }

        let values = bot_values.drain(..).collect_vec();

        let transfer_inst = find_transfer_instr_for_bot(&cur_bot, instructions);
        if let Some(Instruction::Transfer {
            low_target,
            high_target,
            ..
        }) = transfer_inst
        {
            let mut add_target = |tgt: String, val: i64| {
                let values = bots.entry(tgt.clone()).or_insert(Vec::new());
                values.push(val);
                values.sort_unstable();

                if values.len() == 2 {
                    queue.push(tgt);
                }
            };

            add_target(low_target.clone(), values[0]);
            add_target(high_target.clone(), values[1]);
        }
    }

    None
}

fn initial_state(inp: &[Instruction]) -> HashMap<String, Vec<i64>> {
    inp.iter().fold(HashMap::new(), |mut acc, it| {
        if let Instruction::Input { value, target } = it {
            let elem = acc.entry(target.clone()).or_insert(Vec::new());
            elem.push(*value);
            elem.sort_unstable();
        }

        acc
    })
}

#[aoc(day10, part1)]
pub fn part1(inp: &[Instruction]) -> Option<usize> {
    let mut state = initial_state(inp);
    run_instructions_until(inp, &mut state, |it| *it == vec![17, 61])
}

#[aoc(day10, part2)]
pub fn part2(inp: &[Instruction]) -> i64 {
    let mut state = initial_state(inp);
    run_instructions_until(inp, &mut state, |_| false);

    let out0 = state.get("output 0").unwrap();
    let out1 = state.get("output 1").unwrap();
    let out2 = state.get("output 2").unwrap();

    out0[0] * out1[0] * out2[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "value 5 goes to bot 2\n\
                             bot 2 gives low to bot 1 and high to bot 0\n\
                             value 3 goes to bot 1\n\
                             bot 1 gives low to output 1 and high to bot 0\n\
                             bot 0 gives low to output 2 and high to output 0\n\
                             value 2 goes to bot 2";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let mut map = HashMap::new();
        map.insert("bot 2".to_string(), vec![2i64, 5]);
        map.insert("bot 1".to_string(), vec![3]);
        let res = run_instructions_until(&gen, &mut map, |it| *it == vec![2, 5]).unwrap();
        assert_eq!(res, 2);
    }
}
