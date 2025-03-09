use std::{path::PathBuf, process::Command};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    let preprocess_output_filename = args.input.with_extension("i");
    let preprocess_status = Command::new("gcc")
        .arg("-E") // Only run preprocessor
        .arg("-P") // Don't emit linemarkers
        .arg(&args.input)
        .arg("-o")
        .arg(preprocess_output_filename.to_str().unwrap())
        .status()
        .expect("Preprocess command failed to start");
    if !preprocess_status.success() {
        eprintln!("Preprocess failed: {preprocess_status}");
        return Result::Err(());
    }

    // TODO: compile

    let assembler_input_filename = args.input.with_extension("s");
    let assembler_output_filename = assembler_input_filename.with_extension("");
    let assembler_status = Command::new("gcc")
        .arg(&assembler_input_filename)
        .arg("-o")
        .arg(assembler_output_filename.to_str().unwrap())
        .status()
        .expect("Assembler command failed to start");
    if !assembler_status.success() {
        eprintln!("Assembler failed: {assembler_status}");
        return Result::Err(());
    }

    Ok(())
}
