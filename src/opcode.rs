#[derive(Debug, Clone)]
pub enum TwoOpOpcode {
    Mov,
    Read,
    Write,
    Copy,
    Add,
    Sub,
    Mul,
    Div,
    Jrnzdec,
    Lookup,
    Fadd,
    Fsub,
    Fmul,
    Fdiv,
    Imul,
    Idiv,
    Shr,
    Shl,
}

#[derive(Debug, Clone)]
pub enum SingleOpOpcode {
    Call,
    Jmp,
}

#[derive(Debug, Clone)]
pub enum NoOpOpcode {
    Nop,
    Ret,
}
