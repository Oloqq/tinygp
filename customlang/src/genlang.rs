#![allow(unused)]

use crate::token::{tokenize, Token};

type Error = (i32, String);

struct Interpreter {
    memory: Vec<f32>,
    output: Vec<f32>,
    program: Vec<Token>
}

impl Interpreter {
    fn run(&mut self) -> Result<Vec<f32>, Error> {
        todo!();
    }

    fn eval_block() -> Result<(), Error> {
        Ok(())
    }

    fn eval_stat() -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_output() {
        let mut ip = Interpreter {
            memory: vec![],
            output: vec![],
            program: vec![Token::OUTPUT, Token::Const(4)]
        };
        assert_eq!(ip.run().unwrap(), vec![4.0]);
    }
}
