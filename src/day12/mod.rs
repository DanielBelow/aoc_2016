use super::assembunny::{parse_instructions, to_register, Cpu, Instruction};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    parse_instructions(inp)
}

fn execute_instructions(cpu: &mut Cpu, instructions: &[Instruction]) -> i32 {
    while cpu.pc < instructions.len() {
        let inst = instructions[cpu.pc];
        match inst {
            Instruction::Copy(val, r) => {
                let val = val.evaluate(&cpu.registers);
                cpu.registers[to_register(r)] = val;
                cpu.pc += 1;
            }
            Instruction::Inc(r) => {
                cpu.registers[to_register(r)] += 1;
                cpu.pc += 1;
            }
            Instruction::Dec(r) => {
                cpu.registers[to_register(r)] -= 1;
                cpu.pc += 1;
            }
            Instruction::JumpNotZero(val, r) => {
                let val = val.evaluate(&cpu.registers);
                if val == 0 {
                    cpu.pc += 1;
                } else if r < 0 {
                    cpu.pc -= r.abs() as usize;
                } else if r > 0 {
                    cpu.pc += r.abs() as usize;
                }
            }
            Instruction::Toggle(_) => unreachable!(),
        }
    }

    cpu.registers[0]
}

#[aoc(day12, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut cpu = Cpu::default();
    execute_instructions(&mut cpu, instructions)
}

#[aoc(day12, part2)]
pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut cpu = Cpu::default();
    cpu.registers[2] = 1;
    execute_instructions(&mut cpu, instructions)
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
