use std::path::PathBuf;

use dpdl::{
    instructions::{Base64Instruction, Instruction},
    parsing::parse_file,
};

#[test]
fn test_base64_instruction() {
    let ins = Base64Instruction::new();
    let encoded_value = ins.execute(Box::new(String::from("test_file")));
    let correct_value = String::from("dGVzdF9maWxl");
    assert_eq!(
        encoded_value.downcast_ref::<String>().unwrap(),
        &correct_value
    );
}

#[test]
fn test_output() {
    let value = parse_file(PathBuf::from("./tests/test.xml").as_path())
        .unwrap()
        .execute(Box::new(()))
        .downcast_ref::<String>()
        .unwrap()
        .clone();
    assert_eq!(value, String::from("dGVzdF9maWxl"));
}

#[test]
fn test_instruction_tree() {
    //TODO
    let _root_instruction = parse_file(PathBuf::from("./tests/test.xml").as_path()).unwrap();
    assert!(true);
}
