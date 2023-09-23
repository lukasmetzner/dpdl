use std::path::PathBuf;
mod instructions;
pub mod parsing;

fn main() {
    let instruction_tree =
        parsing::parse_file(PathBuf::from("./main-pipeline.xml").as_path()).unwrap();
    println!("{}", instruction_tree.get_instruction_name_tree());
    println!(
        "{}",
        instruction_tree
            .execute(Box::new(()))
            .downcast_ref::<String>()
            .unwrap()
    );
}
