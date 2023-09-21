use std::path::PathBuf;

use dpdl::utils::parse_file;

#[test]
fn check_output() {
    let value = parse_file(PathBuf::from("./tests/proc.xml").as_path())
        .unwrap()
        .execute(Box::new(()))
        .downcast_ref::<String>()
        .unwrap()
        .clone();
    assert_eq!(value, String::from("test_file"));
}

#[test]
fn check_instruction_tree() {
    //TODO
    let root_instruction = parse_file(PathBuf::from("./tests/proc.xml").as_path())
        .unwrap();
    assert!(true);
}