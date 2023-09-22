use std::path::PathBuf;
mod instructions;
pub mod parsing;

fn main() {
    let _ = parsing::parse_file(PathBuf::from("./main-pipeline.xml").as_path())
        .unwrap()
        .execute(Box::new(()))
        .downcast_ref::<String>()
        .unwrap()
        .clone();
}
