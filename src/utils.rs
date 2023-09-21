use std::{error::Error, fs, path::Path};

use roxmltree::Node;

use crate::{
    instruction::Instruction,
    instructions::{
        FileInstruction, InputInstruction, PrintInstruction, RootInstruction, StepInstruction,
    },
};

pub fn parse_children<'a, 'b>(node: &Node<'a, 'b>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for child in node.children() {
        if child.is_text() {
            continue;
        }
        match child.tag_name().name() {
            "input" => instructions.push(Instruction::Input(InputInstruction::new(child.clone()))),
            "step" => instructions.push(Instruction::Step(StepInstruction::new(child.clone()))),
            "file" => instructions.push(Instruction::File(FileInstruction::new(String::from({
                match child.text() {
                    Some(val) => String::from(val),
                    None => String::new(),
                }
            })))),
            "print" => {
                instructions.push(Instruction::Print(PrintInstruction::new(String::from({
                    match child.text() {
                        Some(val) => String::from(val),
                        None => String::new(),
                    }
                }))))
            }
            _ => panic!("unknown xml tag"),
        }
    }
    instructions
}

pub fn parse_file(path: &Path) -> Result<Instruction, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(text.as_str())?;
    let root_element = doc.root_element();
    Ok(Instruction::Root(RootInstruction::new(
        root_element.children(),
    )))
}
