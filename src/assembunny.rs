use parse_display::{Display, FromStr};

#[derive(Default, Copy, Clone, Debug)]
pub struct Cpu {
    pub registers: [i32; 4],
    pub pc: usize,
}

impl Cpu {
    pub fn run_instructions(&mut self, instructions: &mut Vec<Instruction>) -> i32 {
        while self.pc < instructions.len() {
            let inst = instructions[self.pc];
            match inst {
                Instruction::Invalid => {
                    self.pc += 1;
                }
                Instruction::Copy(val, r) => {
                    let val = val.evaluate(&self.registers);
                    self.registers[to_register(r)] = val;
                    self.pc += 1;
                }
                Instruction::Inc(r) => {
                    self.registers[to_register(r)] += 1;
                    self.pc += 1;
                }
                Instruction::Dec(r) => {
                    self.registers[to_register(r)] -= 1;
                    self.pc += 1;
                }
                Instruction::JumpNotZero(val, offset) => {
                    let offset = offset.evaluate(&self.registers);
                    let val = val.evaluate(&self.registers);
                    if val == 0 {
                        self.pc += 1;
                    } else if offset < 0 {
                        self.pc -= offset.unsigned_abs() as usize;
                    } else if offset > 0 {
                        self.pc += offset.unsigned_abs() as usize;
                    }
                }
                Instruction::Toggle(r) => {
                    let offset = self.registers[to_register(r)];
                    let toggle_index = usize::try_from(self.pc as i64 + i64::from(offset)).ok();
                    if let Some(toggle_index) = toggle_index {
                        if toggle_index < instructions.len() {
                            instructions[toggle_index].toggle();
                        }
                    }

                    self.pc += 1;
                }
            }
        }

        self.registers[0]
    }
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("format")]
pub enum Value {
    #[from_str(regex = "(?P<0>[-0-9]+)")]
    Immediate(i32),

    #[from_str(regex = "(?P<0>[A-Za-z])")]
    Register(char),
}

fn to_register(c: char) -> usize {
    match c {
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        _ => unreachable!(),
    }
}

impl Value {
    fn evaluate(self, regs: &[i32; 4]) -> i32 {
        match self {
            Value::Immediate(val) => val,
            Value::Register(r) => regs[to_register(r)],
        }
    }
}
#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("cpy {0} {1}")]
    Copy(Value, char),

    #[display("inc {0}")]
    Inc(char),

    #[display("dec {0}")]
    Dec(char),

    #[display("jnz {0} {1}")]
    JumpNotZero(Value, Value),

    #[display("tgl {0}")]
    Toggle(char),

    Invalid,
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match *self {
            Instruction::Copy(val, r) => {
                let offset = Value::Register(r);
                Instruction::JumpNotZero(val, offset)
            }
            Instruction::Inc(r) => Instruction::Dec(r),
            Instruction::Dec(r) | Instruction::Toggle(r) => Instruction::Inc(r),
            Instruction::JumpNotZero(val, offset) => {
                if let Value::Register(r) = offset {
                    Instruction::Copy(val, r)
                } else {
                    Instruction::Invalid
                }
            }
            Instruction::Invalid => Instruction::Invalid,
        };
    }
}

pub fn parse_instructions(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}
