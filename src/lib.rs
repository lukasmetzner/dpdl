use std::fs;
use std::path::Path;

use roxmltree::Node;

#[derive(Debug)]
pub enum Instruction<'a, 'b> {
    Input(Node<'a, 'b>),
    Step(Node<'a, 'b>),
    File(Node<'a, 'b>),
    Print(Node<'a, 'b>),
}

impl Instruction<'_, '_> {
    pub fn execute(&self) {
        match self {
            Instruction::Input(node) => self._input_ins(node),
            Instruction::Step(node) => self._step_ins(node),
            Instruction::File(node) => self._file_ins(node),
            Instruction::Print(node) => self._print_ins(node),
        }
    }

    fn _input_ins(&self, node: &Node<'_, '_>) { println!("{:?}", node); }
    fn _step_ins(&self, node: &Node<'_, '_>) { println!("{:?}", node); }
    fn _file_ins(&self, node: &Node<'_, '_>) { println!("{:?}", node); }
    fn _print_ins(&self, node: &Node<'_, '_>) { println!("{:?}", node); }
}

pub fn parse_descendants<'a, 'b>(node: &Node<'a, 'b>) -> Vec<Instruction<'a, 'b>> {
    let mut instructions: Vec<Instruction<'a, 'b>> = Vec::new();
    for child in node.children() {
        if child.is_text() {
            continue;
        }
        match child.tag_name().name() {
            "input" => instructions.push(Instruction::Input(child)),
            "step" => instructions.push(Instruction::Step(child)),
            "file" => instructions.push(Instruction::File(child)),
            "print" => instructions.push(Instruction::Print(child)),
            _ => panic!("unknown xml tag"),
        }
        let sub_instructions = parse_descendants(&child);
        instructions.extend(sub_instructions);
    }
    instructions
}

pub fn execute(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    let doc = roxmltree::Document::parse(text.as_str()).unwrap();

    let root_element = doc.root_element();

    let instructions: Vec<Instruction> = parse_descendants(&root_element);
    instructions.into_iter().for_each(|f| f.execute());
}