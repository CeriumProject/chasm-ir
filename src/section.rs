use crate::{Instruction, Words};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub signature: Option<(Words, Vec<(String, Words)>)>,
    pub body: Vec<Instruction>,
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some((result, parameters)) = &self.signature {
            write!(f, "[{result}]")?;
            for (name, size) in parameters {
                write!(f, " {name}[{size}]")?;
            }
        }
        writeln!(f, " {{")?;
        for inst in &self.body {
            inst.write(f, 4)?;
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inst;
    #[test]
    fn section() {
        let section = Section {
            name: String::from("add"),
            signature: Some((1, vec![(String::from("x"), 1), (String::from("y"), 1)])),
            body: vec![Instruction::Result(
                String::from("res"),
                1,
                vec![inst!(Mov, "res", "x"), inst!(Add, "res", "y"), inst!(Ret)],
            )],
        };
        assert_eq!(
            section.to_string().as_str(),
            "add[1] x[1] y[1] {
    result res[1] {
        mov res x
        add res y
        ret
    }
}
"
        );
        println!("{section}");
    }
}
