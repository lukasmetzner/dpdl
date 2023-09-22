use std::{any::Any, fs};

use roxmltree::{Children, Node};

use crate::parsing::parse_children;

use base64::{engine::general_purpose, Engine as _};

pub trait Instruction {
    fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any>;
}

pub struct RootInstruction {
    pub child_instructions: Vec<Box<dyn Instruction>>,
}

impl RootInstruction {
    pub fn new(children: Children<'_, '_>) -> RootInstruction {
        let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
        for child in children {
            if child.is_text() {
                continue;
            }
            instructions.append(&mut parse_children(&child));
        }
        RootInstruction {
            child_instructions: instructions,
        }
    }
}

impl Instruction for RootInstruction {
    fn execute(&self, _value: Box<dyn Any>) -> Box<dyn Any> {
        let mut loop_value: Box<dyn Any> = Box::new(());
        for ele in self.child_instructions.iter() {
            loop_value = ele.execute(loop_value);
        }
        loop_value
    }
}

pub struct InputInstruction {
    pub child_instructions: Vec<Box<dyn Instruction>>,
}

impl InputInstruction {
    pub fn new(child: Node<'_, '_>) -> InputInstruction {
        InputInstruction {
            child_instructions: parse_children(&child),
        }
    }
}

impl Instruction for InputInstruction {
    fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any> {
        let mut loop_value: Box<dyn Any> = value;
        for ele in self.child_instructions.iter() {
            loop_value = ele.execute(loop_value);
        }
        loop_value
    }
}

pub struct StepInstruction {
    pub child_instructions: Vec<Box<dyn Instruction>>,
}

impl StepInstruction {
    pub fn new(child: Node<'_, '_>) -> StepInstruction {
        StepInstruction {
            child_instructions: parse_children(&child),
        }
    }
}

impl Instruction for StepInstruction {
    fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any> {
        let mut loop_value: Box<dyn Any> = value;
        for ele in self.child_instructions.iter() {
            loop_value = ele.execute(loop_value);
        }
        loop_value
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
