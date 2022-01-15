use super::assembunny::{parse_instructions, Cpu, Instruction};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    parse_instructions(inp)
}

#[aoc(day23, part1)]
pub fn part1(insts: &[Instruction]) -> i32 {
    let mut insts = insts.to_vec();
    let mut cpu = Cpu::default();
    cpu.registers[0] = 7;

    cpu.run_instructions(&mut insts)
}

#[aoc(day23, part2)]
pub fn part2(insts: &[Instruction]) -> i32 {
    let mut insts = insts.to_vec();
    let mut cpu = Cpu::default();
    cpu.registers[0] = 12;

    cpu.run_instructions(&mut insts)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "cpy 2 a\n\
                             tgl a\n\
                             tgl a\n\
                             tgl a\n\
                             cpy 1 a\n\
                             dec a\n\
                             dec a";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_DATA);
        let res = part1(&gen);
        assert_eq!(res, 3);
    }
}
