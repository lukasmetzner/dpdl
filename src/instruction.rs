use std::any::Any;

use crate::instructions::{RootInstruction, InputInstruction, StepInstruction, FileInstruction, PrintInstruction};

#[derive(Debug, Clone)]
pub enum Instruction {
    Root(RootInstruction),
    Input(InputInstruction),
    Step(StepInstruction),
    File(FileInstruction),
    Print(PrintInstruction),
}

impl Instruction {
    pub fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any> {
        match self {
            Instruction::Input(instruction) => instruction.execute(value),
            Instruction::Step(instruction) => instruction.execute(value),
            Instruction::File(instruction) => Box::new(instruction.execute()),
            Instruction::Print(instruction) => Box::new(instruction.execute(value)),
            Instruction::Root(instruction) => instruction.execute(),
        }
    }
}
