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
        $crate::Instruction::Reference($crate::Operand::Variable($op1.to_string()), $op2.to_string())
    };
    (rec, $op1:expr, $op2:expr) => {
        $crate::Instruction::Receive(Operand::Variable($op1.to_string()), $op2)
    };

    ($opcode:ident, $op1:expr, $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $crate::Operand::Variable($op1.to_string()), $crate::Operand::Variable($op2.to_string()))
    };
    ($opcode:ident, $op1:expr, val $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $crate::Operand::Variable($op1.to_string()), $crate::Operand::Constant($op2))
    };
    ($opcode:ident, $op1:expr, op $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $crate::Operand::Variable($op1.to_string()), $op2)
    };
    ($opcode:ident, val $op1:expr, $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $crate::Operand::Constant($op1), $crate::Operand::Variable($op2.to_string()))
    };
    ($opcode:ident, val $op1:expr, val $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $crate::Operand::Constant($op1), $crate::Operand::Constant($op2))
    };
    ($opcode:ident, val $op1:expr, op $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $crate::Operand::Constant($op1), $op2)
    };
    ($opcode:ident, op $op1:expr, $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $op1, $crate::Operand::Variable($op2.to_string()))
    };
    ($opcode:ident, op $op1:expr, val $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $op1, $crate::Operand::Constant($op2))
    };
    ($opcode:ident, op $op1:expr, op $op2:expr) => {
        $crate::Instruction::TwoOp($crate::TwoOpOpcode::$opcode, $op1, $op2)
    };

    ($opcode:ident, $op:expr) => {
        $crate::Instruction::SingleOp($crate::SingleOpOpcode::$opcode, $crate::Operand::Variable($op.to_string()))
    };
    ($opcode:ident, val $op:expr) => {
        $crate::Instruction::SingleOp($crate::SingleOpOpcode::$opcode, $crate::Operand::Constant($op))
    };
    ($opcode:ident, op $op:expr) => {
        $crate::Instruction::SingleOp($crate::SingleOpOpcode::$opcode, $op)
    };

    ($opcode:ident) => {
        $crate::Instruction::NoOp($crate::NoOpOpcode::$opcode)
    };
}