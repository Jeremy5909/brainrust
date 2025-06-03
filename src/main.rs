use brainrust::{Instruction, Program};

fn main() {
    let prog = Program::new()
        .add_instruction(Instruction::SetVar("x".to_owned(), 3))
        .add_instruction(Instruction::SetVar("y".to_owned(), 1))
        .add_instruction(Instruction::SetVar("z".to_owned(), 4))
        .add_instruction(Instruction::AddVars("y".to_owned(), "z".to_owned()))
        .add_instruction(Instruction::AddVars("x".to_owned(), "y".to_owned()))
        .debug()
        .build();
    println!("{prog}");
}
