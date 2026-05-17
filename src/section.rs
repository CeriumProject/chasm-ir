use crate::{Instruction, Words};

pub struct Section {
    pub name: String,
    pub signature: Option<(Vec<(String, Words)>, Option<Words>)>,
    pub body: Vec<Instruction>,
}