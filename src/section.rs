use crate::{Instruction, Words};

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub signature: Option<(Words, Vec<(String, Words)>)>,
    pub body: Vec<Instruction>,
}