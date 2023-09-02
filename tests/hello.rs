use std::path::PathBuf;

use dpdl::parse;

#[test]
fn it_works() {
    parse(PathBuf::from("./tests/proc.xml").as_path());
    assert_eq!(4, 4);
}