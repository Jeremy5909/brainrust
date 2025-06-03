use std::mem;

use crate::Program;

pub enum Instruction {
    SetVar(String, i32),
    UnsetVar(String),
}

impl Program {
    pub fn build(mut self) -> String {
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
                    } else {
                        index = self.get_unused_index();
                    }
                    self.debug_msg(&format!("Setting {name} at {index} to {value}"));

                    self.goto(index);
                    self.set_zero();
                    self.add(value);
                    self.vars.insert(name, index);
                }
                Instruction::UnsetVar(name) => {
                    let index = *self.vars.get(&name).unwrap();
                    self.debug_msg(&format!("Unsetting {name} at {index}"));

                    self.goto(index);
                    self.set_zero();
                    self.vars.remove(&name);
                    self.unuse_index();
                }
            }
        }
        self.out
    }
}
