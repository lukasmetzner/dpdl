use std::{any::Any, fs};

use roxmltree::{Node, Children};

use crate::{instruction::Instruction, utils::parse_children};

#[derive(Debug, Clone)]
pub struct RootInstruction {
    pub child_instructions: Vec<Instruction>,
}

impl RootInstruction {
    pub fn new(children: Children<'_, '_>) -> RootInstruction {
        let mut instructions: Vec<Instruction> = Vec::new();
        for child in children {
            instructions.append(&mut parse_children(&child));
        }
        RootInstruction { child_instructions: instructions, }
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
        InputInstruction { child_instructions: parse_children(&child), }
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
        StepInstruction { child_instructions: parse_children(&child), }
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

    pub fn execute(&self) -> String {
        fs::read_to_string(self.value.clone()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct PrintInstruction { 
    pub value: String,
}

impl PrintInstruction {
    pub fn new(value: String) -> PrintInstruction {
        PrintInstruction { value }
    }

    pub fn execute(&self, value: Box<dyn Any>) -> String {
        let string_value = value.downcast_ref::<String>().unwrap().clone();
        println!("{:?}", string_value);
        string_value
    }
}