use std::time::Instant;

use thvm::*;
use Instruction::*;
use I9::*;

fn main() {
    let insts = vec![
        Load(0, 0),
        Load(1, 0),
        Load(2, 1),
        ForPrep(1, 2),
        Mul(3, Reg(0), Reg(1)),
        Mod(0, Reg(3), Cst(2)),
        ForLoop(1, 3),
        Return,
    ];
    let consts = vec![1, 1000000, 100000007];

    execute(insts, consts);
}

fn execute(insts: Vec<Instruction>, consts: Vec<i64>) {
    let mut vm = VM::new();
    let code: Vec<_> = insts
        .into_iter()
        .map(|inst| Word::from(inst).0.to_be_bytes())
        .flatten()
        .collect();

    let start = Instant::now();

    vm.execute(&code, consts).unwrap();

    let duration = Instant::now() - start;

    dbg!(&duration);
}
