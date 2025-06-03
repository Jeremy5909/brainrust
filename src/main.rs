use brainrust::{Instruction, Program};

fn main() {
    let prog = Program::new()
        .add_instruction(Instruction::SetVar("x".to_owned(), 5))
        .debug()
        .build();
    println!("{prog}");
}
