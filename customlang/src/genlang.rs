#![allow(unused)]

use crate::token::{tokenize, Token};

type Error = (usize, String);

struct Interpreter {
    memory: Vec<f32>,
    output: Vec<f32>,
    program: Vec<Token>
}

impl Interpreter {
    fn execute(&mut self) -> Result<Vec<f32>, Error> {
        self.eval_block(0)?;
        return Ok(self.output.clone());
    }

    fn eval_block(&mut self, start: usize) -> Result<(), Error> {
        let mut index = 0;
        while index < self.program.len() {
            index = self.eval_stat(index)?
        }
        Ok(())
    }

    fn eval_stat(&mut self, start: usize) -> Result<usize, Error> {
        match self.program[start] {
            Token::LOAD => {
                let destination = self.program[start + 1]; // this will panic on invalid program right?
                assert!(matches!(destination, Token::Reg(_)));
                match self.program[start + 2] {
                    _ => todo!()
                }
            },
            Token::OUTPUT => {
                match self.program[start + 1] {
                    Token::Const(val) => self.output.push(val),
                    Token::Reg(reg) => self.output.push(self.memory[reg]), // FIXME handle invalid register reference
                    _ => unreachable!() // TODO return meaningful Err
                }
                return Ok(start + 2)
            }
            _ => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_output_1() {
        let mut ip = Interpreter {
            memory: vec![],
            output: vec![],
            program: vec![Token::OUTPUT, Token::Const(4.0)]
        };
        assert_eq!(ip.execute().unwrap(), vec![4.0]);
    }

    #[test]
    fn test_output_2() {
        let mut ip = Interpreter {
            memory: vec![],
            output: vec![],
            program: vec![Token::OUTPUT, Token::Const(5.0), Token::OUTPUT, Token::Const(3.0)]
        };
        assert_eq!(ip.execute().unwrap(), vec![5.0, 3.0]);
    }
}
