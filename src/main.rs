use brainrust::{Instruction, Program};

fn main() {
    let prog = Program::new()
        .add_instruction(Instruction::SetVar("x".to_owned(), 100))
        .debug()
        .build();
    println!("{prog}");
}
