use roxmltree::{Node};

use crate::{Instruction, parse_children};

#[derive(Debug, Clone)]
pub struct InputInstruction { 
    pub child_instructions: Vec<Instruction>,
}
#[derive(Debug, Clone)]
pub struct StepInstruction { 
    pub child_instructions: Vec<Instruction>,
 }
#[derive(Debug, Clone)]
pub struct FileInstruction { 
    pub value: String,
 }
#[derive(Debug, Clone)]
pub struct PrintInstruction { 
    pub value: String,
 }

impl InputInstruction {
    pub fn new(child: Node<'_, '_>) -> InputInstruction {
        InputInstruction { child_instructions: parse_children(&child), }
    }
}

impl StepInstruction {
    pub fn new(child: Node<'_, '_>) -> StepInstruction {
        StepInstruction { child_instructions: parse_children(&child), }
    }
}

impl FileInstruction {
    pub fn new(value: String) -> FileInstruction {
        FileInstruction { value }
    }
}

impl PrintInstruction {
    pub fn new(value: String) -> PrintInstruction {
        PrintInstruction { value }
    }
}