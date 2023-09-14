use std::{fs, any::Any};
use std::path::Path;

pub mod instructions;
pub mod utils;

use instructions::{InputInstruction, StepInstruction, FileInstruction, PrintInstruction};
use utils::parse_children;

#[derive(Debug, Clone)]
pub enum Instruction {
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
        }
    }
}

pub fn execute(path: &Path) -> Box<dyn Any> {
    let text = fs::read_to_string(path).unwrap();
    let doc = roxmltree::Document::parse(text.as_str()).unwrap();

    let root_element = doc.root_element();

    let instructions: Vec<Instruction> = parse_children(&root_element);
    let mut loop_value: Box<dyn Any> = Box::new(());
    for ele in instructions.iter() {
        loop_value = ele.execute(loop_value);
    }
    loop_value
}