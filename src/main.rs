use std::{cmp::Ordering, collections::HashMap, iter, mem};

pub struct Variable {
    index: usize,
    value: u32,
}

pub enum Instruction {
    AddVar(String, u32),
    SetVar(String, u32),
}
pub struct Program {
    instructions: Vec<Instruction>,
    vars: HashMap<String, Variable>,
    index: usize,
    debug: bool,
}

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            vars: HashMap::new(),
            index: 0,
            debug: false,
        }
    }
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }
    pub fn add_instruction(mut self, instruction: Instruction) -> Self {
        self.instructions.push(instruction);
        self
    }
    pub fn build(mut self) -> String {
        let mut out = String::new();
        let mut last_used_index = 0;
        for instruction in mem::take(&mut self.instructions) {
            match instruction {
                Instruction::AddVar(name, value) => {
                    last_used_index += 1;
                    out.push_str(&self.goto(last_used_index));
                    self.vars.insert(
                        name,
                        Variable {
                            index: last_used_index,
                            value,
                        },
                    );

                    out.push_str(&self.add(value as i32))
                }
                Instruction::SetVar(name, value) => {
                    let var = self.vars.get(&name).unwrap();
                    let index = var.index;
                    let var_value = var.value;
                    out.push_str(&self.goto(index));
                    out.push_str(&self.add(value as i32 - var_value as i32));
                }
            }
        }

        out
    }
    fn add(&self, n: i32) -> String {
        let c = match n.signum() {
            1 => '+',
            -1 => '-',
            _ => return String::new(),
        };
        let out = iter::repeat_n(c, n.abs() as usize).collect();
        if self.debug {
            eprintln!("Adding {n}");
            eprintln!("{out}");
        }
        out
    }
    fn goto(&mut self, i: usize) -> String {
        let c = match self.index.cmp(&i) {
            Ordering::Less => '>',
            Ordering::Greater => '<',
            Ordering::Equal => return String::new(),
        };
        let out = iter::repeat_n(c, (self.index as i32 - i as i32).abs() as usize).collect();
        if self.debug {
            eprintln!("Going to {i}");
            eprintln!("{out}");
        };
        self.index = i;
        out
    }
}

fn main() {
    let prog = Program::new()
        .add_instruction(Instruction::AddVar("x".to_owned(), 3))
        .add_instruction(Instruction::AddVar("y".to_owned(), 4))
        .add_instruction(Instruction::AddVar("z".to_owned(), 1))
        .add_instruction(Instruction::SetVar("x".to_owned(), 1))
        .add_instruction(Instruction::SetVar("y".to_owned(), 2))
        .add_instruction(Instruction::SetVar("z".to_owned(), 3))
        .debug()
        .build();
    println!("{prog}");
}
