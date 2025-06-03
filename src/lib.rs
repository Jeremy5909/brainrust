use std::{collections::HashMap, fmt::Debug};

pub use instructions::Instruction;

// Implementing for Instruction enum
mod instructions;
// Methods that actually add to the out
mod actions;
// Other useful stuff
mod utils;

#[derive()]
pub enum Error {
    VariableNotFound(String),
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VariableNotFound(name) => write!(f, "variable '{name}' not found"),
        }
    }
}
enum Optimization {
    Speed,
    Size,
    None,
}
pub struct Program {
    instructions: Vec<Instruction>,
    vars: HashMap<String, usize>,
    index: usize,
    debug: bool,
    used_indexes: Vec<usize>,
    out: String,
    optimization: Optimization,
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
            optimization: Optimization::None,
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
    pub fn optimize_speed(mut self) -> Self {
        self.optimization = Optimization::Speed;
        self
    }
    pub fn size(mut self) -> Self {
        self.optimization = Optimization::Size;
        self
    }
}
