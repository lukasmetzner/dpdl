use std::{
    any::{type_name, Any},
    fs,
};

use roxmltree::Node;

use crate::parsing::parse_children;

use base64::{engine::general_purpose, Engine as _};

pub trait Instruction {
    fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any>;
    fn print_instruction(&self) {
        println!("{:?}", type_name::<Self>());
    }
}

pub struct GroupInstruction {
    pub child_instructions: Vec<Box<dyn Instruction>>,
}

impl GroupInstruction {
    pub fn new(node: Node) -> GroupInstruction {
        GroupInstruction {
            child_instructions: parse_children(&node),
        }
    }
}

impl Instruction for GroupInstruction {
    fn execute(&self, _value: Box<dyn Any>) -> Box<dyn Any> {
        let mut loop_value: Box<dyn Any> = _value;
        for ele in self.child_instructions.iter() {
            loop_value = ele.execute(loop_value);
        }
        loop_value
    }

    fn print_instruction(&self) {
        println!("{:?}", type_name::<Self>());
        for child in self.child_instructions.iter() {
            child.print_instruction();
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileInstruction {
    pub value: String,
}

impl FileInstruction {
    pub fn new(value: String) -> FileInstruction {
        FileInstruction { value }
    }
}

impl Instruction for FileInstruction {
    fn execute(&self, _value: Box<dyn Any>) -> Box<dyn Any> {
        Box::new(fs::read_to_string(self.value.clone()).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct PrintInstruction {}

impl PrintInstruction {
    pub fn new() -> PrintInstruction {
        PrintInstruction {}
    }
}

impl Instruction for PrintInstruction {
    fn execute(&self, _value: Box<dyn Any>) -> Box<dyn Any> {
        let string_value = _value.downcast_ref::<String>().unwrap().clone();
        println!("{:?}", string_value);
        Box::new(string_value)
    }
}

#[derive(Debug, Clone)]
pub struct Base64Instruction {}

impl Base64Instruction {
    pub fn new() -> Base64Instruction {
        Base64Instruction {}
    }
}

impl Instruction for Base64Instruction {
    fn execute(&self, _value: Box<dyn Any>) -> Box<dyn Any> {
        let string_value = _value.downcast_ref::<String>().unwrap();
        let base64_encoded_value = general_purpose::STANDARD.encode(string_value);
        Box::new(base64_encoded_value)
    }
}
