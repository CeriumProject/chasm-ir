#[derive(Debug, Clone)]
pub enum Operand {
    Constant(u16),
    Variable(String),
}