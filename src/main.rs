use std::path::PathBuf;

mod instruction;
mod instructions;
pub mod utils;

fn main() {
    let _ = utils::parse_file(PathBuf::from("./tests/proc.xml").as_path())
        .unwrap()
        .execute(Box::new(()))
        .downcast_ref::<String>()
        .unwrap()
        .clone();
}
