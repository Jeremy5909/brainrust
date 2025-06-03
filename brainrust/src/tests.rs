use crate::{Instruction, Program};

#[test]
fn set_and_change_three_vars() {
    let prog = Program::new()
        .add_instruction(Instruction::SetVar("x".to_owned(), 3))
        .add_instruction(Instruction::SetVar("y".to_owned(), 4))
        .add_instruction(Instruction::SetVar("z".to_owned(), 1))
        .add_instruction(Instruction::SetVar("x".to_owned(), 1))
        .add_instruction(Instruction::SetVar("y".to_owned(), 2))
        .add_instruction(Instruction::SetVar("z".to_owned(), 3))
        .debug()
        .build();
    assert_eq!(prog, "+++>++++>+<<-->-->++")
}

#[test]
fn unset_vars_and_replace() {
    let prog = Program::new()
        .add_instruction(Instruction::SetVar("x".to_owned(), 3))
        .add_instruction(Instruction::SetVar("y".to_owned(), 4))
        .add_instruction(Instruction::SetVar("z".to_owned(), 1))
        .add_instruction(Instruction::UnsetVar("x".to_owned()))
        .add_instruction(Instruction::UnsetVar("y".to_owned()))
        .add_instruction(Instruction::SetVar("a".to_owned(), 7))
        .add_instruction(Instruction::SetVar("b".to_owned(), 7))
        .add_instruction(Instruction::SetVar("c".to_owned(), 7))
        .debug()
        .build();
    assert_eq!(prog, "+++>++++>+<<--->----<+++++++>+++++++>>+++++++")
}
