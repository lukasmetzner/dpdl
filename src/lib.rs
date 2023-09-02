use std::fs;
use std::path::Path;

use roxmltree::{Node, Children};

#[derive(Debug)]
pub enum Instruction<'a, 'b> {
    Input(Children<'a, 'b>),
    Step(Children<'a, 'b>),
}

impl Instruction<'_, '_> {
    pub fn execute(&self) {
        match self {
            Instruction::Input(_) => println!("{:?}", self),
            Instruction::Step(_) => println!("{:?}", self),
        }
    }
}

fn _parse_body<'a, 'b>(body: Node<'a, 'b>) -> Vec<Instruction<'a, 'b>> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for descendant in body.children() {
        if descendant.is_text() { continue };
        match descendant.tag_name().name() {
            "input" => instructions.push(Instruction::Input(descendant.children())),
            "step" => instructions.push(Instruction::Step(descendant.children())),
            _ => panic!("unkown xml tag"),
        }
    }
    instructions
}

pub fn parse(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    let doc = roxmltree::Document::parse(text.as_str()).unwrap();

    let root_element = doc.root_element();

    let mut instructions: Vec<Instruction> = Vec::new();

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
    
    for ins in instructions {
        ins.execute();
    }
}