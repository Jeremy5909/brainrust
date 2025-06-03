use brainrust::{Instruction, Program};

fn main() {
    let prog = Program::new()
        .add_instruction(Instruction::AddString("x".to_owned(), "Hello!".to_owned()))
        .debug()
        .build();
    println!("{prog}");
}
