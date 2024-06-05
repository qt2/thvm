macro_rules! define_consts {
    ($($x:ident),+) => {
        define_consts!{0; $($x),*}
    };
    ($i:expr; $x:ident, $($y:ident),*) => {
        const $x: u8 = $i;
        define_consts!{$i+1; $($y),*}
    };
    ($i:expr; $x:ident) => {
        const $x: u8 = $i;
    };
}

define_consts!(OP_RETURN, OP_MOVE, OP_LOAD, OP_FORPREP, OP_FORLOOP, OP_MUL, OP_MOD);

#[derive(Debug, Clone)]
pub enum Instruction {
    Return,
    Move(u8, I9),
    Load(u8, u16),
    ForPrep(u8, u16),
    ForLoop(u8, u16),
    Mul(u8, I9, I9),
    Mod(u8, I9, I9),
}

impl From<Instruction> for Word {
    fn from(inst: Instruction) -> Self {
        match inst {
            Instruction::Return => Word::new(OP_RETURN),
            Instruction::Move(a, b) => Word::with_a_b(OP_MOVE, a, b),
            Instruction::Load(a, bx) => Word::with_a_bx(OP_LOAD, a, bx),
            Instruction::ForPrep(a, bx) => Word::with_a_bx(OP_FORPREP, a, bx),
            Instruction::ForLoop(a, bx) => Word::with_a_bx(OP_FORLOOP, a, bx),
            Instruction::Mul(a, b, c) => Word::with_a_b_c(OP_MUL, a, b, c),
            Instruction::Mod(a, b, c) => Word::with_a_b_c(OP_MOD, a, b, c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Word(pub u32);

impl Word {
    pub fn new(opcode: u8) -> Self {
        Word((opcode as u32) << 26)
    }

    #[inline]
    pub fn set_a(mut self, a: u8) -> Self {
        self.0 |= (a as u32) << 18;
        self
    }

    #[inline]
    pub fn set_b(mut self, b: I9) -> Self {
        self.0 |= u32::from(b) << 9;
        self
    }

    #[inline]
    pub fn set_c(mut self, c: I9) -> Self {
        self.0 |= u32::from(c);
        self
    }

    #[inline]
    pub fn set_bx(mut self, bx: u16) -> Self {
        self.0 |= u32::from(bx);
        self
    }

    #[inline]
    pub fn set_sbx(mut self, bx: u16, neg: bool) -> Self {
        self.0 |= u32::from(bx);
        if neg {
            self.0 |= 1 << 17;
        }
        self
    }

    pub fn with_a(opcode: u8, a: u8) -> Self {
        Word::new(opcode).set_a(a)
    }

    pub fn with_a_b(opcode: u8, a: u8, b: I9) -> Self {
        Word::new(opcode).set_a(a).set_b(b)
    }

    pub fn with_a_b_c(opcode: u8, a: u8, b: I9, c: I9) -> Self {
        Word::new(opcode).set_a(a).set_b(b).set_c(c)
    }

    pub fn with_a_bx(opcode: u8, a: u8, bx: u16) -> Self {
        Word::new(opcode).set_a(a).set_bx(bx)
    }

    pub fn with_a_sbx(opcode: u8, a: u8, bx: u16, neg: bool) -> Self {
        Word::new(opcode).set_a(a).set_sbx(bx, neg)
    }

    #[inline]
    pub fn get_opcode(self) -> u8 {
        (self.0 >> 26) as u8
    }

    #[inline]
    pub fn parse_a(self) -> u8 {
        (self.0 >> 18 & 0xff) as u8
    }

    #[inline]
    pub fn parse_a_b(self) -> (u8, I9) {
        (self.parse_a(), I9::from(self.0 >> 9))
    }

    pub fn parse_a_b_c(self) -> (u8, I9, I9) {
        let (a, b) = self.parse_a_b();
        (a, b, I9::from(self.0))
    }

    pub fn parse_a_bx(self) -> (u8, u16) {
        (self.parse_a(), self.0 as u16)
    }

    pub fn parse_a_sbx(self) -> (u8, u16, bool) {
        (self.parse_a(), self.0 as u16, (self.0 >> 17 & 1) == 1)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum I9 {
    Reg(u8),
    Cst(u8),
}

impl From<I9> for u32 {
    fn from(value: I9) -> Self {
        match value {
            I9::Reg(x) => x as u32,
            I9::Cst(x) => x as u32 | 1 << 8,
        }
    }
}

impl From<u32> for I9 {
    fn from(value: u32) -> Self {
        let flag = value >> 8 & 1;
        let x = (value & 0xff) as u8;
        match flag {
            0 => I9::Reg(x),
            1 => I9::Cst(x),
            _ => unreachable!(),
        }
    }
}

pub struct VM {
    reg: Vec<i64>,
    pc: usize,
    cst: Vec<i64>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            reg: Vec::new(),
            pc: 0,
            cst: Vec::new(),
        }
    }

    pub fn execute(&mut self, code: &[u8], consts: Vec<i64>) -> Result<(), ()> {
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
                    self.insert(a, self.cst[bx as usize]);
                }
                OP_MUL => {
                    let (a, b, c) = word.parse_a_b_c();
                    self.insert(a, self.load(b) * self.load(c));
                }
                OP_MOD => {
                    let (a, b, c) = word.parse_a_b_c();
                    self.insert(a, self.load(b) % self.load(c));
                }
                OP_FORPREP => {
                    let (a, bx) = word.parse_a_bx();
                    self.reg[a as usize] -= 1;
                    self.pc += bx as usize;
                }
                OP_FORLOOP => {
                    let (a, bx) = word.parse_a_bx();
                    self.reg[a as usize] += 1;

                    if self.reg[a as usize] < self.reg[a as usize + 1] {
                        self.pc -= bx as usize;
                    }
                }
                _ => unimplemented!(),
            }

            self.pc += 1;
        }

        Ok(())
    }

    fn insert(&mut self, i: u8, v: i64) {
        let i = i as usize;
        if self.reg.len() == i {
            self.reg.push(v);
        } else {
            self.reg[i] = v;
        }
    }

    fn load(&self, addr: I9) -> i64 {
        match addr {
            I9::Reg(x) => self.reg[x as usize],
            I9::Cst(x) => self.cst[x as usize],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    use I9::*;

    fn execute(insts: Vec<Instruction>, consts: Vec<i64>) {
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
        let consts = vec![2, 3];

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
        let consts = vec![1, 2, 4];

        execute(insts, consts);
    }
}
