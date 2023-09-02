use std::path::PathBuf;

use dpdl::parse;

#[test]
fn it_works() {
    parse(PathBuf::from("./tests/proc.xml")
        .as_path())
        .iter()
        .for_each(|f| f.execute());
    assert_eq!(4, 4);
}