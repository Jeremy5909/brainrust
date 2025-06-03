use std::{cmp::Ordering, collections::HashMap, iter, mem};

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
    fn debug_msg(&self, msg: &str) {
        if self.debug {
            eprintln!("{msg}");
        }
    }
    fn unuse_index(&mut self) {
        let i = self
            .used_indexes
            .iter()
            .position(|&x| x == self.index)
            .unwrap();
        self.used_indexes.remove(i);
    }
    fn get_unused_index(&mut self) -> usize {
        let mut i = 0;
        loop {
            if !self.used_indexes.contains(&(i)) {
                self.used_indexes.push(i);
                return i;
            } else {
                i += 1;
            }
        }
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
    fn add(&mut self, n: i32) {
        let c = match n.signum() {
            1 => '+',
            -1 => '-',
            _ => return,
        };
        let out: String = iter::repeat_n(c, n.abs() as usize).collect();
        self.out.push_str(&out);
    }
    fn goto(&mut self, i: usize) {
        let c = match self.index.cmp(&i) {
            Ordering::Less => '>',
            Ordering::Greater => '<',
            Ordering::Equal => return,
        };
        let out: String =
            iter::repeat_n(c, (self.index as i32 - i as i32).abs() as usize).collect();
        self.index = i;
        self.out.push_str(&out);
    }
}
