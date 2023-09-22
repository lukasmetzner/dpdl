use std::any::Any;

use crate::instructions::{
    Base64Instruction, FileInstruction, InputInstruction, PrintInstruction, RootInstruction,
    StepInstruction,
};

#[derive(Debug, Clone)]
pub enum Instruction {
    Root(RootInstruction),
    Input(InputInstruction),
    Step(StepInstruction),
    File(FileInstruction),
    Print(PrintInstruction),
    Base64(Base64Instruction),
}

impl Instruction {
    pub fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any> {
        match self {
            Instruction::Root(instruction) => instruction.execute(),

            Instruction::Input(instruction) => instruction.execute(value),
            Instruction::Step(instruction) => instruction.execute(value),

            Instruction::File(instruction) => instruction.execute(),
            Instruction::Print(instruction) => instruction.execute(value),
            Instruction::Base64(instruction) => instruction.execute(value),
        }
    }
}
