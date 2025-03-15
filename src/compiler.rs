use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow;

use crate::{asm, lexer::Lexer, parser::Parser};

#[derive(Clone, Copy)]
pub enum Phase {
    Lex,
    Parse,
    Codegen,
}

pub struct Compiler {
    final_phase: Option<Phase>,
    pretty_print: bool,
    sources: Vec<PathBuf>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            final_phase: None,
            pretty_print: false,
            sources: Vec::new(),
        }
    }

    pub fn with_final_phase(mut self, phase: Option<Phase>) -> Self {
        self.final_phase = phase;
        self
    }

    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }

    pub fn add_source(mut self, path: PathBuf) -> Self {
        self.sources.push(path);
        self
    }

    pub fn run(&self) -> anyhow::Result<()> {
        for filename in &self.sources {
            let Ok(file_contents) = fs::read_to_string(filename) else {
                let msg = format!("Failed to read file {}", filename.display());
                return Err(anyhow::anyhow!(msg));
            };

            let tokens = Lexer::new(file_contents).run()?;
            if self.pretty_print {
                let path = filename.with_extension("lex");
                let f = fs::File::create(path).expect("Failed to create file");
                let mut file_writer = BufWriter::new(f);

                let mut curr_line = 0;
                for token in &tokens {
                    for i in curr_line + 1..=token.loc.line {
                        writeln!(file_writer, "")?;
                        write!(file_writer, "Line {i}: ")?;
                    }
                    curr_line = token.loc.line;

                    write!(file_writer, "{} ", token.kind)?;
                }
            }

            if matches!(self.final_phase, Some(Phase::Lex)) {
                continue;
            }

            let ast = Parser::new(tokens).parse_ast()?;
            if self.pretty_print {
                let path = filename.with_extension("ast");
                let f = fs::File::create(path).expect("Failed to create file");
                let mut file_writer = BufWriter::new(f);
                writeln!(file_writer, "{ast:#?}")?;
            }

            if matches!(self.final_phase, Some(Phase::Parse)) {
                continue;
            }

            let asm = asm::Program::from(ast);
            if self.pretty_print {
                let path = filename.with_extension("asm");
                let f = fs::File::create(path).expect("Failed to create file");
                let mut file_writer = BufWriter::new(f);
                writeln!(file_writer, "{asm:#?}")?;
            }

            // TODO: codegen
        }

        Ok(())
    }
}
