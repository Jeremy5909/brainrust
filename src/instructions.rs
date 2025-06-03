use std::mem;

use crate::{Error, Program};

pub enum Instruction {
    SetVar(String, i32),
    UnsetVar(String),
    Sum(String, String),
    AddString(String, String),
}

impl Program {
    pub fn build(mut self) -> Result<String, Error> {
        for instruction in mem::take(&mut self.instructions) {
            self.debug_msg("\n");
            self.debug_msg(&format!("Variables: {:#?}", self.vars));
            self.debug_msg(&format!("Used indexes: {:?}", self.used_indexes));
            self.debug_msg("-----------------------\n");

            match instruction {
                Instruction::SetVar(name, value) => {
                    let index;
                    if let Some(var) = self.vars.get(&name) {
                        index = *var;
                        self.goto(index);
                        self.set_zero();
                    } else {
                        index = self.allocate();
                        self.goto(index);
                    }
                    self.debug_msg(&format!("Setting {name} at {index} to {value}"));
                    self.add(value);
                    self.vars.insert(name, index);
                }
                Instruction::AddString(name, val) => {
                    let start = self.allocate_arr(val.len());

                    for (i, c) in val.chars().enumerate() {
                        self.goto(start + i);
                        self.add(c as i32);
                    }

                    self.vars.insert(name, start);
                }
                Instruction::UnsetVar(name) => {
                    let index = *self
                        .vars
                        .get(&name)
                        .ok_or(Error::VariableNotFound(name.clone()))?;
                    self.debug_msg(&format!("Unsetting {name} at {index}"));

                    self.goto(index);
                    self.set_zero();
                    self.vars.remove(&name);
                    self.deallocate();
                }
                Instruction::Sum(var1, var2) => {
                    let index1 = *self.vars.get(&var1).ok_or(Error::VariableNotFound(var1))?;
                    let index2 = *self.vars.get(&var2).ok_or(Error::VariableNotFound(var2))?;
                    // TODO the list making should be handled in actions.rs
                    self.goto(index2);
                    self.out.push_str("[");
                    self.goto(index1);
                    self.out.push_str("+");
                    self.goto(index2);
                    self.out.push_str("-");
                    self.out.push_str("]");
                }
            }
        }
        Ok(self.out)
    }
}
