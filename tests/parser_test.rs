use std::path::PathBuf;

use dpdl::parse_file;

#[test]
fn it_works() {
    let value = parse_file(PathBuf::from("./tests/proc.xml").as_path())
        .unwrap()
        .execute(Box::new(()))
        .downcast_ref::<String>()
        .unwrap()
        .clone();
    assert_eq!(value, String::from("test_file"));
}