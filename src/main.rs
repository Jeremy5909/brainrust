use brainrust::{Instruction, Program};

fn main() {
    let prog = Program::new()
        .add_instruction(Instruction::SetVar("x".to_owned(), 5))
        .add_instruction(Instruction::SetVar("y".to_owned(), 6))
        .add_instruction(Instruction::UnsetVar("x".to_owned()))
        .add_instruction(Instruction::SetVar("x".to_owned(), 6))
        .debug()
        .build();
    println!("{prog}");
}
