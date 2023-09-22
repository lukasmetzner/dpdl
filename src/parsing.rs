use std::{error::Error, fs, path::Path};

use roxmltree::Node;

use crate::instructions::{
    Base64Instruction, FileInstruction, InputInstruction, Instruction, PrintInstruction,
    RootInstruction, StepInstruction,
};

pub fn parse_children<'a, 'b>(node: &Node<'a, 'b>) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    for child in node.children() {
        if child.is_text() {
            continue;
        }

        match child.tag_name().name() {
            "input" => instructions.push(Box::new(InputInstruction::new(child.clone()))),
            "step" => instructions.push(Box::new(StepInstruction::new(child.clone()))),
            "file" => instructions.push(Box::new(FileInstruction::new(String::from({
                match child.text() {
                    Some(val) => String::from(val),
                    None => String::new(),
                }
            })))),
            "print" => instructions.push(Box::new(PrintInstruction::new())),
            "base64" => instructions.push(Box::new(Base64Instruction::new())),
            _ => panic!("unknown xml tag"),
        }
    }
    instructions
}

pub fn parse_file(path: &Path) -> Result<Box<dyn Instruction>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(text.as_str())?;
    let root_element = doc.root_element();
    Ok(Box::new(RootInstruction::new(root_element.children())))
}
