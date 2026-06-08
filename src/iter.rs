use crate::{Instruction, Section};

pub struct InstIter<'a> {
    stack: Vec<std::slice::Iter<'a, Instruction>>,
}

impl<'a> Iterator for InstIter<'a> {
    type Item = &'a Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.last_mut()?.next() {
            None => {
                self.stack.pop();
                self.next()
            }
            Some(instruction) => {
                if let Instruction::Alloc(_, _, inner)
                | Instruction::Param(_, _, inner)
                | Instruction::Result(_, _, inner) = instruction
                {
                    self.stack.push(inner.iter());
                }
                Some(instruction)
            }
        }
    }
}

pub trait IntoInstIter {
    fn iter_rec(&'_ self) -> InstIter<'_>;
}

impl IntoInstIter for Vec<Instruction> {
    fn iter_rec(&'_ self) -> InstIter<'_> {
        InstIter { stack: vec![self.iter()] }
    }
}

impl IntoInstIter for Section {
    fn iter_rec(&'_ self) -> InstIter<'_> {
        InstIter { stack: vec![self.body.iter()] }
    }
}