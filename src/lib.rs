use std::fs;
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
    pub fn execute(&self) {
        match self {
            Instruction::Input(instruction) => self._input_ins(instruction),
            Instruction::Step(instruction) => self._step_ins(instruction),
            Instruction::File(instruction) => self._file_ins(instruction),
            Instruction::Print(instruction) => self._print_ins(instruction),
        }
    }

    fn _input_ins(&self, instruction: &InputInstruction) { instruction.child_instructions.iter().for_each(|f| f.execute()); }
    fn _step_ins(&self, instruction: &StepInstruction) { instruction.child_instructions.iter().for_each(|f| f.execute()); }
    fn _file_ins(&self, instruction: &FileInstruction) { println!("{:?}", instruction.value); }
    fn _print_ins(&self, instruction: &PrintInstruction) { println!("{:?}", instruction.value); }
}

pub fn execute(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    let doc = roxmltree::Document::parse(text.as_str()).unwrap();

    let root_element = doc.root_element();

    let instructions: Vec<Instruction> = parse_children(&root_element);
    instructions.into_iter().for_each(|f| f.execute());
}