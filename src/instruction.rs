use crate::{NoOpOpcode, Operand, SingleOpOpcode, TwoOpOpcode, Words};

#[derive(Debug, Clone)]
pub enum Instruction {
    Sublabel(String),
    Alloc(String, Words, Vec<Instruction>),
    Param(String, Words, Vec<Instruction>),
    Result(String, Words, Vec<Instruction>),
    /// Used to take references. Stores memory location of variable in operand.
    Reference(Operand, String),
    /// Stores nth result of called function in operand.
    Receive(Operand, usize),
    TwoOp(TwoOpOpcode, Operand, Operand),
    SingleOp(SingleOpOpcode, Operand),
    NoOp(NoOpOpcode),
    RawWords(Vec<Operand>),
}