use std::mem;

use crate::{Error, Program};

pub enum Instruction {
    SetVar(String, i32),
    UnsetVar(String),
    Sum(String, String),
    SetString(String, String),
    PrintString(String),
    Copy(String, String),
    Multiply(String, String),
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
                    self.deallocate(index);
                }
                Instruction::Sum(var1, var2) => {
                    let index1 = self.get_var(&var1)?;
                    let index2 = self.get_var(&var2)?;
                    // TODO the list making should be handled in actions.rs
                    self.add_vars(index1, index2);
                }
                Instruction::Copy(target, new) => {
                    let from_index = self.get_var(&target)?;

                    // Make sure [new] isn't a variable already
                    self.get_var(&new).unwrap_err();

                    let copy = self.copy(from_index);
                    self.vars.insert(new, copy);
                }
                Instruction::Multiply(var1, var2) => {
                    let val1 = self.get_var(&var1)?;
                    let val2 = self.get_var(&var2)?;

                    self.out.push('[');

                    let copy1 = self.copy(val2);
                    let copy2 = self.copy(copy1);
                    let result = self.allocate();
                    self.add_vars(result, copy2);
                    self.add_vars(result, copy1);

                    self.goto(val1);
                    self.add(-1);
                    self.goto(val2);

                    self.out.push(']');
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
    // TODO i want one thing for assembly like stuff (memcpy, idk) and you define where the output
    // should go or something or maybe it just goes one place and one for C level stuff (mult,
    // maybe this is where u define where output goes like the mem )
    fn copy(&mut self, original: usize) -> usize {
        let copy1 = self.allocate();
        let copy2 = self.allocate();
        // Reset to if somethings already there
        self.goto(original);
        self.out.push_str("["); //TODO yk
        self.goto(copy1);
        self.add(1);
        self.goto(copy2);
        self.add(1);
        self.goto(original);
        self.add(-1);
        self.out.push_str("]");

        self.add_vars(original, copy2);

        copy1
    }
    fn add_vars(&mut self, v1: usize, v2: usize) {
        self.goto(v2);
        self.out.push('[');
        self.goto(v1);
        self.add(1);
        self.goto(v2);
        self.add(-1);
        self.out.push(']');
        self.deallocate(v2);
    }
}
