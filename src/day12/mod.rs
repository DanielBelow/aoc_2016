use super::assembunny::{parse_instructions, Cpu, Instruction};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    parse_instructions(inp)
}

fn execute_instructions(cpu: &mut Cpu, instructions: &mut Vec<Instruction>) -> i32 {
    cpu.run_instructions(instructions)
}

#[aoc(day12, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut instructions = instructions.to_vec();
    let mut cpu = Cpu::default();
    execute_instructions(&mut cpu, &mut instructions)
}

#[aoc(day12, part2)]
pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut instructions = instructions.to_vec();
    let mut cpu = Cpu::default();
    cpu.registers[2] = 1;
    execute_instructions(&mut cpu, &mut instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "cpy 41 a\n\
                             inc a\n\
                             inc a\n\
                             dec a\n\
                             jnz a 2\n\
                             dec a";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 42);
    }
}
