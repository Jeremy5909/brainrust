use std::{collections::HashMap, mem};

// Methods that actually add to the out
mod actions;
// Other useful stuff
mod util;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Variable {
    index: usize,
    value: u32,
}

pub enum Instruction {
    SetVar(String, u32),
    UnsetVar(String),
}
pub struct Program {
    instructions: Vec<Instruction>,
    vars: HashMap<String, Variable>,
    index: usize,
    debug: bool,
    used_indexes: Vec<usize>,
    out: String,
    optimised: bool,
}

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            vars: HashMap::new(),
            index: 0,
            debug: false,
            used_indexes: Vec::new(),
            out: String::new(),
            optimised: false,
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
    pub fn optimised(mut self) -> Self {
        self.optimised = true;
        self
    }
    pub fn build(mut self) -> String {
        for instruction in mem::take(&mut self.instructions) {
            self.debug_msg("\n");
            self.debug_msg(&format!("Variables: {:#?}", self.vars));
            self.debug_msg(&format!("Used indexes: {:?}", self.used_indexes));
            self.debug_msg("-----------------------\n");

            match instruction {
                Instruction::SetVar(name, value) => {
                    let mut curr_value = 0;
                    let index;
                    if let Some(var) = self.vars.get(&name) {
                        curr_value = var.value;
                        index = var.index;
                    } else {
                        index = self.get_unused_index();
                    }
                    self.debug_msg(&format!("Setting {name} at {index} to {value}"));

                    self.goto(index);
                    self.add(value as i32 - curr_value as i32);
                    self.vars.insert(name, Variable { index, value });
                }
                Instruction::UnsetVar(name) => {
                    let var = self.vars.get(&name).unwrap();
                    let index = var.index;
                    let var_value = var.value;
                    self.debug_msg(&format!("Unsetting {name} at {index}"));

                    self.goto(index);
                    self.add(-(var_value as i32));
                    self.vars.remove(&name);
                    self.unuse_index();
                }
            }
        }
        self.out
    }
}
