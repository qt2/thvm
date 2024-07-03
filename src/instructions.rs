macro_rules! define_consts {
    ($($x:ident),+) => {
        define_consts!{0; $($x),*}
    };
    ($i:expr; $x:ident, $($y:ident),*) => {
        pub const $x: u8 = $i;
        define_consts!{$i+1; $($y),*}
    };
    ($i:expr; $x:ident) => {
        pub const $x: u8 = $i;
    };
}

define_consts!(OP_RETURN, OP_MOVE, OP_LOAD, OP_FORPREP, OP_FORLOOP, OP_MUL, OP_MOD, OP_LOADGLB);

#[derive(Debug, Clone)]
pub enum Instruction {
    Return,
    Move(u8, I9),
    Load(u8, u16),
    ForPrep(u8, u16),
    ForLoop(u8, u16),
    /// Multiply. R(A) = RC(B) * RC(C)
    Mul(u8, I9, I9),
    /// Modulus. R(A) = RC(B) % RC(C)
    Mod(u8, I9, I9),
    /// Load globals. R(A) = Globals\[RC(B)]
    LoadGlb(u8, I9),
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
            Instruction::LoadGlb(a, b) => Word::with_a_b(OP_LOADGLB, a, b),
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
