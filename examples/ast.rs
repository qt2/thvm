use thvm::{Instruction, Value};

pub enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Assign(String, Box<Expr>),
    Lit(Value),
}

pub enum BinOp {
    Mul,
    Mod,
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Compiler {}
    }

    pub fn compile(&mut self, exprs: Vec<Expr>) -> Vec<Instruction> {
        let mut insts = Vec::new();

        for expr in exprs {}

        insts
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        use Expr::*;

        let exprs = vec![Assign(
            "a".into(),
            Box::new(Binary(BinOp::Mul, Box::new(Lit(2)), Box::new(Lit(3)))),
        )];

        let mut compiler = Compiler::new();
        compiler.compile(exprs);
    }
}
