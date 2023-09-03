use std::fs;
use std::path::Path;

use roxmltree::Node;

#[derive(Debug)]
pub enum Instruction<'a, 'b> {
    Input(Node<'a, 'b>),
    Step(Node<'a, 'b>),
    File(Node<'a, 'b>),
}

impl Instruction<'_, '_> {
    pub fn execute(&self) {
        match self {
            Instruction::Input(node) => println!("{:?}", node),
            Instruction::Step(node) => println!("{:?}", node),
            Instruction::File(node) => println!("{:?}", node),
        }
    }
}

// pub fn parse_desendants(node: &Node) -> Vec<Instruction> {
//     let instructions: Vec<Instruction> = Vec::new();
//     instructions
// }

pub fn execute(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    let doc = roxmltree::Document::parse(text.as_str()).unwrap();

    let root_element = doc.root_element();

    let mut instructions: Vec<Instruction> = Vec::new();
    for descendant in root_element.children() {
        if descendant.is_text() { continue };
        match descendant.tag_name().name() {
            "input" => instructions.push(Instruction::Input(descendant)),
            "step" => instructions.push(Instruction::Step(descendant)),
            "file" => instructions.push(Instruction::Step(descendant)),
            _ => panic!("unkown xml tag"),
        }
    }
    
    for instruction in instructions {
        instruction.execute();
    }
}