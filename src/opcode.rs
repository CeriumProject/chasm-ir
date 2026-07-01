use std::fmt::Display;

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
    Jrnz,
    Jrz,
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
    Itof,
    Utof,
    Ftoi,
    Ftou,
    Ctx,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
pub enum SingleOpOpcode {
    Call,
    Jmp,
    Dbg,
}

#[derive(Debug, Clone)]
pub enum NoOpOpcode {
    Nop,
    Ret,
    Send,
}

impl Display for TwoOpOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for SingleOpOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for NoOpOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
