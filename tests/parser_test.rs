use std::path::PathBuf;

use dpdl::execute;

#[test]
fn it_works() {
    let value = execute(PathBuf::from("./tests/proc.xml").as_path())
        .downcast_ref::<String>()
        .unwrap()
        .clone();
    assert_eq!(value, String::from("test_file"));
}