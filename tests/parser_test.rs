use std::path::PathBuf;

use dpdl::parsing::parse_file;

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
