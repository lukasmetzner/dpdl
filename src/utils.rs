use roxmltree::Node;

use crate::{Instruction, instructions::{StepInstruction, FileInstruction, PrintInstruction, InputInstruction}};

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
            "print" => instructions.push(Instruction::Print(PrintInstruction::new(String::from({
                match child.text() {
                    Some(val) => String::from(val),
                    None => String::new(),
                }
            })))),
            _ => panic!("unknown xml tag"),
        }
    }
    instructions
}