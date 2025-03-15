use std::fmt::Write;

use crate::asm;

const INDENT: &str = "    ";

pub struct Codegen<'a> {
    asm: &'a asm::Program,
    output: String,
}

impl<'a> Codegen<'a> {
    pub fn new(asm: &'a asm::Program) -> Self {
        Self {
            asm,
            output: String::new(),
        }
    }
    pub fn emit(mut self) -> anyhow::Result<String> {
        self.emit_program(self.asm)?;
        Ok(self.output)
    }

    fn emit_program(&mut self, program: &asm::Program) -> anyhow::Result<()> {
        self.emit_function(&program.function)?;
        writeln!(
            self.output,
            "{INDENT}.section .note.GNU-stack,\"\",@progbits"
        )?;
        Ok(())
    }

    fn emit_function(&mut self, function: &asm::Function) -> anyhow::Result<()> {
        writeln!(self.output, "{INDENT}.globl {}", function.name)?;
        writeln!(self.output, "{}:", function.name)?;
        for inst in &function.instructions {
            self.emit_instruction(inst)?;
        }
        writeln!(self.output)?;
        Ok(())
    }

    fn emit_instruction(&mut self, inst: &asm::Instruction) -> anyhow::Result<()> {
        match inst {
            asm::Instruction::Mov(src, dst) => {
                write!(self.output, "{INDENT}movl ")?;
                self.emit_operand(src)?;
                write!(self.output, ", ")?;
                self.emit_operand(dst)?;
                writeln!(self.output)?;
            }
            asm::Instruction::Ret => {
                writeln!(self.output, "{INDENT}ret")?;
            }
        }

        Ok(())
    }

    fn emit_operand(&mut self, op: &asm::Operand) -> anyhow::Result<()> {
        match op {
            asm::Operand::Immediate(i) => write!(self.output, "${i}")?,
            asm::Operand::Register => write!(self.output, "%eax")?,
        }
        Ok(())
    }
}
