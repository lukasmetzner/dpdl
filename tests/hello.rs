use std::path::PathBuf;

use dpdl::execute;

#[test]
fn it_works() {
    execute(PathBuf::from("./tests/proc.xml").as_path());
    assert_eq!(4, 4);
}