#[derive(Debug)]
pub enum TwoOpOpcode {
    Mov,
    Read,
    Write,
    Add,
}

#[derive(Debug)]
pub enum SingleOpOpcode {
    Call,
}

#[derive(Debug)]
pub enum NoOpOpcode {
    Ret,
}
