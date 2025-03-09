use std::{path::PathBuf, process::Command};

use anyhow;
use clap::Parser;

use crate::compiler::{Compiler, Phase};

mod compiler;
mod lexer;

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

    #[arg(long)]
    pretty: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let final_phase = match (args.lex, args.parse, args.codegen) {
        (_, _, true) => Some(Phase::Codegen),
        (_, true, _) => Some(Phase::Parse),
        (true, _, _) => Some(Phase::Lex),
        _ => None,
    };

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
        let msg = format!("Preprocess failed: {preprocess_status}");
        return Err(anyhow::anyhow!(msg));
    }

    Compiler::new()
        .with_final_phase(final_phase)
        .with_pretty_print(args.pretty)
        .add_source(preprocess_output_filename)
        .run()?;

    if final_phase.is_none() {
        let assembler_input_filename = args.input.with_extension("s");
        let assembler_output_filename = assembler_input_filename.with_extension("");
        let assembler_status = Command::new("gcc")
            .arg(&assembler_input_filename)
            .arg("-o")
            .arg(assembler_output_filename.to_str().unwrap())
            .status()
            .expect("Assembler command failed to start");
        if !assembler_status.success() {
            let msg = format!("Assembler failed: {assembler_status}");
            return Err(anyhow::anyhow!(msg));
        }
    }

    Ok(())
}
