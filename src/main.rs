use std::path::PathBuf;
mod instructions;
pub mod parsing;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(index=1)]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    let file_path_buf = PathBuf::from(args.file_path);
    let instruction_tree = parsing::parse_file(file_path_buf.as_path()).unwrap();
    println!("{}", instruction_tree.get_instruction_name_tree());
    println!(
        "{}",
        instruction_tree
            .execute(Box::new(()))
            .downcast_ref::<String>()
            .unwrap()
    );
}
