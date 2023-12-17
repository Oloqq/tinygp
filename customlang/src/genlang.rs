#![allow(unused)]

use crate::token::{tokenize, Token};

type Error = (usize, String);

struct Interpreter {
    memory: Vec<f32>,
    output: Vec<f32>,
    program: Vec<Token>,
}

fn protected_div(lhs: f32, rhs: f32) -> f32 {
    const THRESHOLD: f32 = 0.001;
    if rhs <= THRESHOLD {
        lhs
    }
    else {
        lhs / rhs
    }
}

impl Interpreter {
    fn execute(&mut self) -> Result<Vec<f32>, Error> {
        self.eval_block(0)?;
        return Ok(self.output.clone());
    }

    fn execute_expr(&self) -> Result<f32, Error> {
        let (index, val) = self.eval_expr(0).unwrap();
        Ok(val)
    }

    fn eval_expr(&self, index: usize) -> Result<(usize, f32), Error> {
        let token = self.program[index];
        if let Token::Const(val) = token {
            return Ok((index + 1, val));
        } else if let Token::Reg(reg) = token {
            todo!();
        }
        else if matches!(token, Token::SIN | Token::COS) { // TODO | Token::NOT
            todo!();
        }
        else {
            let (i, lhs) = self.eval_expr(index + 1).unwrap();
            let (i, rhs) = self.eval_expr(i).unwrap();
            let result = match token {
                Token::Const(_) => unreachable!(),
                Token::Reg(_) => unreachable!(),
                Token::ADD => lhs + rhs,
                Token::SUB => lhs - rhs,
                Token::MUL => lhs * rhs,
                Token::DIV => protected_div(lhs, rhs),
                Token::SIN => unreachable!(),
                Token::COS => unreachable!(),
                Token::LOAD => unreachable!(),
                Token::OUTPUT => unreachable!(),
            };
            return Ok((i, result));
        }
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
                    _ => todo!(),
                }
            }
            Token::OUTPUT => {
                match self.program[start + 1] {
                    Token::Const(val) => self.output.push(val),
                    Token::Reg(reg) => self.output.push(self.memory[reg]), // TODO handle invalid register reference
                    _ => unreachable!(), // TODO return meaningful Err
                }
                return Ok(start + 2);
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
            program: vec![Token::OUTPUT, Token::Const(4.0)],
        };
        assert_eq!(ip.execute().unwrap(), vec![4.0]);
    }

    #[test]
    fn test_output_2() {
        let mut ip = Interpreter {
            memory: vec![],
            output: vec![],
            program: vec![
                Token::OUTPUT,
                Token::Const(5.0),
                Token::OUTPUT,
                Token::Const(3.0),
            ],
        };
        assert_eq!(ip.execute().unwrap(), vec![5.0, 3.0]);
    }

    #[test]
    #[ignore]
    fn test_idiot_proofness() {
        let mut ip = Interpreter {
            memory: vec![],
            output: vec![],
            program: vec![],
        };

        ip.program = vec![Token::OUTPUT, Token::Const(5.0), Token::OUTPUT];
        assert_eq!(ip.execute().unwrap(), vec![5.0, 3.0]);
    }

    #[test]
    fn test_expression() {
        let mut ip = Interpreter {
            memory: vec![],
            output: vec![],
            program: vec![],
        };

        ip.program = vec![Token::ADD, Token::Const(5.0), Token::Const(3.0)];
        assert_eq!(ip.execute_expr().unwrap(), 8.0);

        ip.program = vec![Token::SUB, Token::Const(5.0), Token::Const(8.0)];
        assert_eq!(ip.execute_expr().unwrap(), -3.0);

        ip.program = vec![Token::MUL, Token::Const(5.0), Token::Const(3.0)];
        assert_eq!(ip.execute_expr().unwrap(), 15.0);

        ip.program = vec![Token::DIV, Token::Const(6.0), Token::Const(2.0)];
        assert_eq!(ip.execute_expr().unwrap(), 3.0);

        ip.program = vec![Token::DIV, Token::Const(6.0), Token::Const(0.0)];
        assert_eq!(ip.execute_expr().unwrap(), 6.0);
    }
}
