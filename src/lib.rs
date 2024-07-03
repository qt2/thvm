pub mod errors;
pub mod instructions;
mod register;

pub use crate::{instructions::*, register::*};
use errors::{RuntimeError, RuntimeErrorKind};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    Str(String),
}

impl Value {
    pub fn num(&self) -> f64 {
        match self {
            Value::Num(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn str(&self) -> &str {
        match self {
            Value::Str(v) => v,
            _ => unreachable!(),
        }
    }
}

pub struct VM {
    reg: Registers,
    pc: usize,
    cst: Vec<Value>,
    glb: FxHashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            reg: Registers::new(),
            pc: 0,
            cst: Vec::new(),
            glb: FxHashMap::default(),
        }
    }

    pub fn execute(&mut self, code: &[u8], consts: Vec<Value>) -> Result<(), RuntimeErrorKind> {
        use Value::*;

        self.pc = 0;
        self.cst = consts;

        loop {
            let word = Word(u32::from_be_bytes(
                code[4 * self.pc..4 * self.pc + 4].try_into().unwrap(),
            ));
            let opcode = word.get_opcode();

            match opcode {
                OP_RETURN => break,
                OP_MOVE => break,
                OP_LOAD => {
                    let (a, bx) = word.parse_a_bx();
                    self.reg.insert(a, self.cst[bx as usize].clone());
                }
                OP_MUL => {
                    let (a, b, c) = word.parse_a_b_c();
                    let value = match (self.load(b), self.load(c)) {
                        (Num(b), Num(c)) => Num(b * c),
                        _ => return Err(RuntimeErrorKind::InvalidOperator),
                    };
                    self.reg.insert(a, value);
                }
                OP_MOD => {
                    let (a, b, c) = word.parse_a_b_c();
                    let value = match (self.load(b), self.load(c)) {
                        (Num(b), Num(c)) => Num(b % c),
                        _ => return Err(RuntimeErrorKind::InvalidOperator),
                    };
                    self.reg.insert(a, value);
                }
                OP_FORPREP => {
                    let (a, bx) = word.parse_a_bx();
                    match self.reg[a] {
                        Num(ref mut v) => {
                            *v -= 1.0;
                        }
                        _ => unreachable!(),
                    }
                    self.pc += bx as usize;
                }
                OP_FORLOOP => {
                    let (a, bx) = word.parse_a_bx();
                    let lim = self.reg[a + 1].num();
                    match self.reg[a] {
                        Num(ref mut v) => {
                            *v += 1.0;
                            if *v < lim {
                                self.pc -= bx as usize;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                OP_LOADGLB => {
                    let (a, b) = word.parse_a_b();
                    let name_obj = self.load(b);
                    let name = name_obj.str();
                    let value = self
                        .glb
                        .get(name)
                        .ok_or_else(|| RuntimeErrorKind::GlobalNotFound(name.into()))?
                        .clone();
                    self.reg.insert(a, value);
                }
                _ => unimplemented!(),
            }

            self.pc += 1;
        }

        Ok(())
    }

    fn load(&self, addr: I9) -> Value {
        match addr {
            I9::Reg(x) => self.reg[x].clone(),
            I9::Cst(x) => self.cst[x as usize].clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    use Value::*;
    use I9::*;

    fn execute(insts: Vec<Instruction>, consts: Vec<Value>) {
        let mut vm = VM::new();
        let code: Vec<_> = insts
            .into_iter()
            .map(|inst| Word::from(inst).0.to_be_bytes())
            .flatten()
            .collect();

        vm.execute(&code, consts).unwrap();

        dbg!(&vm.reg);
    }

    #[test]
    fn test_mul() {
        let insts = vec![Load(0, 0), Mul(1, Reg(0), Cst(1)), Return];
        let consts = vec![Num(2.0), Num(3.0)];

        execute(insts, consts);
    }

    #[test]
    fn test_for() {
        let insts = vec![
            Load(0, 0),
            Load(1, 0),
            Load(2, 2),
            ForPrep(1, 1),
            Mul(0, Reg(0), Cst(1)),
            ForLoop(1, 2),
            Return,
        ];
        let consts = vec![Num(1.0), Num(2.0), Num(4.0)];

        execute(insts, consts);
    }
}
