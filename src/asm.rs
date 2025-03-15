use crate::ast;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

impl From<ast::Program> for Program {
    fn from(value: ast::Program) -> Self {
        Self {
            function: value.function.into(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

impl From<ast::Function> for Function {
    fn from(value: ast::Function) -> Self {
        Self {
            name: value.ident,
            instructions: value.stmt.into(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

impl From<ast::Stmt> for Vec<Instruction> {
    fn from(value: ast::Stmt) -> Self {
        vec![
            Instruction::Mov(Operand::Immediate(value.expr.int), Operand::Register),
            Instruction::Ret,
        ]
    }
}

#[derive(Debug)]
pub enum Operand {
    Immediate(i32),
    Register,
}
