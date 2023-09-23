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
fn test_get_instruction_name_tree() {
    let root_instruction = parse_file(PathBuf::from("./tests/test.xml").as_path()).unwrap();
    let instruction_name_tree = root_instruction.get_instruction_name_tree();
    assert_eq!(instruction_name_tree.root, "GroupInstruction");

    assert_eq!(instruction_name_tree.leaves[0].root, "GroupInstruction");
    assert_eq!(instruction_name_tree.leaves[1].root, "GroupInstruction");

    assert_eq!(instruction_name_tree.leaves[0].leaves[0].root, "FileInstruction");
    assert_eq!(instruction_name_tree.leaves[1].leaves[0].root, "Base64Instruction");
}
