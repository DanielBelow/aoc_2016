use parse_display::{Display, FromStr};

#[derive(Default, Copy, Clone, Debug)]
pub struct Cpu {
    pub registers: [i32; 4],
    pub pc: usize,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("format")]
pub enum Value {
    #[from_str(regex = "(?P<0>[-0-9]+)")]
    Immediate(i32),

    #[from_str(regex = "(?P<0>[A-Za-z])")]
    Register(char),
}

pub fn to_register(c: char) -> usize {
    match c {
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        _ => unreachable!(),
    }
}

impl Value {
    pub fn evaluate(self, regs: &[i32; 4]) -> i32 {
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
    JumpNotZero(Value, i32),

    #[display("tgl {0}")]
    Toggle(char),
}

pub fn parse_instructions(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}
