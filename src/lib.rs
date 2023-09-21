use std::error::Error;
use std::path::Path;
use std::{any::Any, fs};

pub mod instructions;
pub mod utils;

use instructions::{
    FileInstruction, InputInstruction, PrintInstruction, RootInstruction, StepInstruction,
};
use utils::parse_children;

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

pub fn parse_file(path: &Path) -> Result<Instruction, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(text.as_str())?;
    let root_element = doc.root_element();
    Ok(parse_children(&root_element).first().unwrap().clone())
}
