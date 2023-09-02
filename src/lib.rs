use std::fs;
use std::path::Path;

use roxmltree::Node;

pub trait Instruction { 
    fn execute(&self);
}

#[derive(Clone, Debug)]
pub struct StepInstruction {}

#[derive(Clone, Debug)]
pub struct InputInstruction {}

impl Instruction for StepInstruction {
    fn execute(&self) {
        println!("Step");
    }
}

impl Instruction for InputInstruction {
    fn execute(&self) {
        println!("Input");
    }
}

fn _parse_body(body: Node<'_, '_>) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    for descendant in body.children() {
        if descendant.is_text() { continue };
        match descendant.tag_name().name() {
            "input" => instructions.push(Box::new(InputInstruction{})),
            "step" => instructions.push(Box::new(StepInstruction{})),
            _ => panic!("unkown xml tag"),
        }
    }
    instructions
}

pub fn parse(path: &Path) -> Vec<Box<dyn Instruction>> {
    let text = fs::read_to_string(path).unwrap();
    let doc = roxmltree::Document::parse(text.as_str()).unwrap();

    let root_element = doc.root_element();

    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();

    for descendant in root_element.children() {
        if descendant.is_text() { continue; }
        if descendant.is_comment() { continue; }
        let instructions_subset = match descendant.tag_name().name() {
            "head" => todo!(),
            "body" => _parse_body(descendant),
            _ => panic!("unkown xml tag"),
        };
        instructions.extend(instructions_subset);
    }
    instructions
}