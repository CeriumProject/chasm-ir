use std::fmt::{Display, Formatter};
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
    Definition(String, Operand),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write(f, 0)
    }
}

impl Instruction {
    pub(crate) fn write(&self, f: &mut Formatter<'_>, indent: usize) -> std::fmt::Result {
        let indent = match self {
            Instruction::Sublabel(_) => indent.saturating_sub(4),
            _ => indent,
        };
        let prefix = " ".repeat(indent);
        match self {
            Instruction::Sublabel(label) => writeln!(f, "{prefix}.{label}:"),
            Instruction::Alloc(name, size, content) => {
                writeln!(f, "{prefix}alloc {name}[{size}] {{")?;
                for inst in content {
                    inst.write(f, indent + 4)?;
                }
                writeln!(f, "{prefix}}}")
            }
            Instruction::Param(name, size, content) => {
                writeln!(f, "{prefix}param {name}[{size}] {{")?;
                for inst in content {
                    inst.write(f, indent + 4)?;
                }
                writeln!(f, "{prefix}}}")
            }
            Instruction::Result(name, size, content) => {
                writeln!(f, "{prefix}result {name}[{size}] {{")?;
                for inst in content {
                    inst.write(f, indent + 4)?;
                }
                writeln!(f, "{prefix}}}")
            }
            Instruction::Reference(dst, variable) => writeln!(f, "{prefix}ref {dst:?}, {variable}"),
            Instruction::Receive(dst, offset) => writeln!(f, "{prefix}rec {dst:?}, {offset}"),
            Instruction::TwoOp(opcode, op1, op2) => writeln!(f, "{prefix}{opcode} {op1:?} {op2:?}"),
            Instruction::SingleOp(opcode, op1) => writeln!(f, "{prefix}{opcode} {op1:?}"),
            Instruction::NoOp(opcode) => writeln!(f, "{prefix}{opcode}"),
            Instruction::RawWords(words) => {
                write!(f, "{prefix}dw")?;
                for word in words {
                    write!(f, " {word:?}")?;
                }
                writeln!(f, "")
            },
            Instruction::Definition(alias, value) => writeln!(f, "{prefix}define {alias} {value:?}"),
        }
    }
}