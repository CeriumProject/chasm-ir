mod section;
mod operand;
mod instruction;
mod opcode;

pub use section::Section;
pub use operand::Operand;
pub use instruction::Instruction;
pub use opcode::{TwoOpOpcode, SingleOpOpcode, NoOpOpcode};

pub type Words = usize;

#[macro_export]
macro_rules! inst {
    (ref, $op1:expr, $op2:expr) => {
        Instruction::Reference(Operand::Variable($op1.to_string()), $op2.to_string())
    };
    (rec, $op1:expr, $op2:expr) => {
        Instruction::Receive(Operand::Variable($op1.to_string()), $op2)
    };
    ($opcode:ident, $op1:expr, $op2:expr) => {
        Instruction::TwoOp(TwoOpOpcode::$opcode, Operand::Variable($op1.to_string()), Operand::Variable($op2.to_string()))
    };
    ($opcode:ident, $op1:expr, val $op2:expr) => {
        Instruction::TwoOp(TwoOpOpcode::$opcode, Operand::Variable($op1.to_string()), Operand::Constant($op2))
    };
    ($opcode:ident, val $op1:expr, val $op2:expr) => {
        Instruction::TwoOp(TwoOpOpcode::$opcode, Operand::Constant($op1), Operand::Constant($op2))
    };
    ($opcode:ident, val $op1:expr, $op2:expr) => {
        Instruction::TwoOp(TwoOpOpcode::$opcode, Operand::Constant($op1), Operand::Variable($op2.to_string()))
    };
    ($opcode:ident, $op:expr) => {
        Instruction::SingleOp(SingleOpOpcode::$opcode, Operand::Variable($op.to_string()))
    };
    ($opcode:ident, val $op:expr) => {
        Instruction::SingleOp(SingleOpOpcode::$opcode, Operand::Constant($op))
    };
    ($opcode:ident) => {
        Instruction::NoOp(NoOpOpcode::$opcode)
    };
}