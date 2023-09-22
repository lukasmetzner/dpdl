use std::{any::Any, fs};

use roxmltree::{Children, Node};

use crate::{instruction::Instruction, parsing::parse_children};

use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Clone)]
pub struct RootInstruction {
    pub child_instructions: Vec<Instruction>,
}

impl RootInstruction {
    pub fn new(children: Children<'_, '_>) -> RootInstruction {
        let mut instructions: Vec<Instruction> = Vec::new();
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

    pub fn execute(&self) -> Box<dyn Any> {
        let mut loop_value: Box<dyn Any> = Box::new(());
        for ele in self.child_instructions.iter() {
            loop_value = ele.execute(loop_value);
        }
        loop_value
    }
}

#[derive(Debug, Clone)]
pub struct InputInstruction {
    pub child_instructions: Vec<Instruction>,
}

impl InputInstruction {
    pub fn new(child: Node<'_, '_>) -> InputInstruction {
        InputInstruction {
            child_instructions: parse_children(&child),
        }
    }

    pub fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any> {
        let mut loop_value: Box<dyn Any> = value;
        for ele in self.child_instructions.iter() {
            loop_value = ele.execute(loop_value);
        }
        loop_value
    }
}

#[derive(Debug, Clone)]
pub struct StepInstruction {
    pub child_instructions: Vec<Instruction>,
}

impl StepInstruction {
    pub fn new(child: Node<'_, '_>) -> StepInstruction {
        StepInstruction {
            child_instructions: parse_children(&child),
        }
    }

    pub fn execute(&self, value: Box<dyn Any>) -> Box<dyn Any> {
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

    pub fn execute(&self) -> Box<String> {
        Box::new(fs::read_to_string(self.value.clone()).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct PrintInstruction {}

impl PrintInstruction {
    pub fn new() -> PrintInstruction {
        PrintInstruction {}
    }

    pub fn execute(&self, value: Box<dyn Any>) -> Box<String> {
        let string_value = value.downcast_ref::<String>().unwrap().clone();
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

    pub fn execute(&self, value: Box<dyn Any>) -> Box<String> {
        let string_value = value.downcast_ref::<String>().unwrap();
        let base64_encoded_value = general_purpose::STANDARD.encode(string_value);
        Box::new(base64_encoded_value)
    }
}
