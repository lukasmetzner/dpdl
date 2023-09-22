use std::{error::Error, fs, path::Path};

use roxmltree::Node;

use crate::instructions::{
    Base64Instruction, FileInstruction, InputInstruction, Instruction, PrintInstruction,
    RootInstruction, StepInstruction,
};

pub fn parse_children(node: &Node) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    for child in node.children() {
        if child.is_text() {
            continue;
        }
        let instruction: Box<dyn Instruction> = match child.tag_name().name() {
            "input" => Box::new(InputInstruction::new(child.clone())),
            "step" => Box::new(StepInstruction::new(child.clone())),
            "file" => {
                let text_val = child.text().unwrap_or_default().to_string();
                Box::new(FileInstruction::new(text_val))
            }
            "print" => Box::new(PrintInstruction::new()),
            "base64" => Box::new(Base64Instruction::new()),
            _ => panic!("unknown xml tag"),
        };
        instructions.push(instruction);
    }
    instructions
}

pub fn parse_file(path: &Path) -> Result<Box<dyn Instruction>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(text.as_str())?;
    let root_element = doc.root_element();
    Ok(Box::new(RootInstruction::new(root_element)))
}
