#[derive(Debug)]
pub enum Operand {
    Constant(u16),
    Variable(String),
}