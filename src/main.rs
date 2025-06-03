use brainrust::{Instruction, Program};

mod brainrust;

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
