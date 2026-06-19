use std::fmt::Debug;

#[derive(Clone)]
pub enum Operand {
    Constant(u16),
    Variable(String),
}

impl Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Operand::Constant(constant) => write!(f, "#{constant}"),
            Operand::Variable(variable) => write!(f, "{variable}"),
        }
    }
}
