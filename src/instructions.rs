use std::mem;

use crate::{Error, Program};

pub enum Instruction {
    SetVar(String, i32),
    UnsetVar(String),
    Sum(String, String),
    SetString(String, String),
    PrintString(String),
    Copy(String, String),
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
                    if let Ok(var) = self.get_var(&name) {
                        index = var;
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
                Instruction::SetString(name, val) => {
                    let start = self.allocate_str(val.len());

                    for (i, c) in val.chars().enumerate() {
                        self.goto(start + i);
                        self.add(c as i32);
                    }
                    self.goto(start + val.len());

                    self.vars.insert(name, start);
                }
                Instruction::UnsetVar(name) => {
                    let index = self.get_var(&name)?;
                    self.debug_msg(&format!("Unsetting {name} at {index}"));

                    self.goto(index);
                    self.set_zero();
                    self.vars.remove(&name);
                    self.deallocate();
                }
                Instruction::Sum(var1, var2) => {
                    let index1 = self.get_var(&var1)?;
                    let index2 = self.get_var(&var2)?;
                    // TODO the list making should be handled in actions.rs
                    self.goto(index2);
                    self.out.push('[');
                    self.goto(index1);
                    self.out.push('+');
                    self.goto(index2);
                    self.out.push('-');
                    self.out.push(']');
                }
                Instruction::Copy(target, new) => {
                    let from_index = self.get_var(&target)?;

                    // Make sure [new] isn't a variable already
                    self.get_var(&new).unwrap_err();

                    let (index_1, index_2) = self.copy(from_index);
                    self.vars.insert(target, index_1);
                    self.vars.insert(new, index_2);
                }
                Instruction::PrintString(name) => {
                    let start = self.get_var(&name)?;
                    self.goto(start);
                    // all this hould be not here cuz its too low level
                    // doesnt move self.index
                    self.out.push_str(".[>.]");
                }
            }
        }
        Ok(self.out)
    }
    fn copy(&mut self, target: usize) -> (usize, usize) {
        let out1 = self.allocate();
        let out2 = self.allocate();
        // Reset to if somethings already there
        self.goto(target);
        self.out.push_str("["); //TODO yk
        self.goto(out1);
        self.add(1);
        self.goto(out2);
        self.add(1);
        self.goto(target);
        self.add(-1);
        self.out.push_str("]");

        // target now 0 and unused
        self.goto(target);
        self.deallocate();

        (out1, out2)
    }
}
