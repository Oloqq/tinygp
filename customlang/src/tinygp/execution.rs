use crate::params::Params;

use super::common::*;

#[allow(unused)]
#[derive(Debug)]
enum EvalError {
    Finished,
    Syntax,
    Semantic,
}

struct Context {
    memory: Vec<f32>,
    input: Vec<f32>,
    output: Vec<f32>,
    input_cursor: usize,
}

impl Context {
    pub fn new(memsize: usize, input: Vec<f32>) -> Self {
        Context {
            memory: vec![0.0; memsize],
            input,
            output: Vec::new(),
            input_cursor: 0,
        }
    }

    pub fn next_input(&mut self) -> Option<f32> {
        if self.input.len() < self.input_cursor {
            let val = self.input[self.input_cursor];
            self.input_cursor += 1;
            Some(val)
        } else {
            None
        }
    }
}

pub fn execute(program: &Program, params: &Params) -> f32 {
    log::trace!("executing {:?}", program);
    let mut ctx = Context::new(params.memsize, vec![]);
    let mut cursor = 0;

    match eval_stat(program, &mut cursor, &mut ctx) {
        Ok(_) => {}
        Err(e) => match e {
            EvalError::Finished => {}
            EvalError::Syntax => todo!(),
            EvalError::Semantic => todo!(),
        },
    }
    return *ctx.output.get(0).unwrap_or(&1.0);
}

fn read_reg(token: Token, memory: &Vec<f32>) -> f32 {
    match token {
        Token::Reg(num) => memory.get(num).unwrap().clone(),
        _ => {
            unreachable!()
        }
    }
}

fn eval_stat(program: &Program, cursor: &mut usize, ctx: &mut Context) -> Result<(), EvalError> {
    if let Token::Stat(stat) = program[*cursor] {
        match stat {
            Stat::OUTPUT => {
                let regval = read_reg(program[*cursor + 1], &ctx.memory);
                ctx.output.push(regval);
            }
            Stat::INPUT => {
                let regnum = match program[*cursor + 1] {
                    Token::Reg(num) => num,
                    _ => panic!(
                        "Expected Reg at {}, got {:?}",
                        *cursor + 1,
                        program[*cursor + 1]
                    ),
                };
                let val = match ctx.next_input() {
                    Some(val) => val,
                    None => return Err(EvalError::Finished),
                };
                ctx.memory[regnum] = val;
            }
        }
        return Ok(());
    }
    panic!("called eval_stat on non-stat");
}

#[allow(unused)]
fn eval_expr(program: &Program, memory: &Vec<f32>, cursor: &mut usize) -> f32 {
    let opcode = program[*cursor];
    *cursor += 1;

    return match opcode {
        Token::Expr(func) => match func {
            Expr::ADD => eval_expr(program, memory, cursor) + eval_expr(program, memory, cursor),
            Expr::SUB => eval_expr(program, memory, cursor) - eval_expr(program, memory, cursor),
            Expr::MUL => eval_expr(program, memory, cursor) * eval_expr(program, memory, cursor),
            Expr::DIV => {
                let numerator = eval_expr(program, memory, cursor);
                let denominator = eval_expr(program, memory, cursor);
                if denominator.abs() <= 0.001 {
                    numerator
                } else {
                    numerator / denominator
                }
            }
            Expr::SIN => f32::sin(eval_expr(program, memory, cursor)),
            Expr::COS => f32::cos(eval_expr(program, memory, cursor)),
            _ => unreachable!(),
        },
        Token::Reg(i) => memory[i],
        Token::Stat(_) => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let program: Vec<Token> = vec![
            Token::Expr(Expr::ADD),
            Token::Reg(0),
            Token::Expr(Expr::DIV),
            Token::Reg(1),
            Token::Reg(1),
        ];
        let data = vec![1.0, -2.0];
        assert_eq!(2.0, eval_expr(&program, &data, &mut 0));

        let program: Vec<Token> = vec![
            Token::Expr(Expr::SUB),
            Token::Reg(0),
            Token::Expr(Expr::DIV),
            Token::Reg(1),
            Token::Reg(2),
        ];
        assert_eq!(
            0.8776571,
            eval_expr(
                &program,
                &vec![0.0, -4.025456902691228, 4.58659426408455],
                &mut 0
            )
        );
    }
}