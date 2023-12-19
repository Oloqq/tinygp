use super::common::*;

#[allow(unused)]
#[derive(Debug)]
enum EvalError {
    Finished,
    Syntax,
    Semantic,
}

pub struct Runtime {
    memory: Vec<f32>,
    input: Vec<f32>,
    output: Vec<f32>,
    input_cursor: usize,
}

impl Runtime {
    pub fn new(memsize: usize, input: Vec<f32>) -> Self {
        Runtime {
            memory: vec![0.0; memsize],
            input,
            output: Vec::new(),
            input_cursor: 0,
        }
    }

    pub fn next_input(&mut self) -> Option<f32> {
        if self.input_cursor < self.input.len()  {
            let val = self.input[self.input_cursor];
            self.input_cursor += 1;
            Some(val)
        } else {
            None
        }
    }
}

pub fn execute_with_new_runtime(program: &Program, memsize: usize) -> f32 {
    log::trace!("executing {:?}", program);
    let mut runtime = Runtime::new(memsize, vec![]);

    execute(program, &mut runtime);
    return *runtime.output.get(0).unwrap_or(&1.0);
}

pub fn execute(program: &Program, runtime: &mut Runtime) {
    match eval_stat(program, 0, runtime) {
        Ok(_) => {}
        Err(e) => match e {
            EvalError::Finished => {}
            EvalError::Syntax => todo!(),
            EvalError::Semantic => todo!(),
        },
    }
}

fn read_reg(token: Token, memory: &Vec<f32>) -> f32 {
    match token {
        Token::Reg(num) => memory.get(num).unwrap().clone(),
        _ => {
            unreachable!()
        }
    }
}

fn eval_stat(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    if let Token::Stat(stat) = program[pos] {
        return match stat {
            Stat::OUTPUT => {
                let regval = read_reg(program[pos + 1], &runtime.memory);
                println!("{}", regval);
                runtime.output.push(regval);
                Ok(pos + 1)
            }
            Stat::INPUT => {
                let regnum = match program[pos + 1] {
                    Token::Reg(num) => num,
                    _ => panic!(
                        "Expected Reg at {}, got {:?}",
                        pos + 1,
                        program[pos + 1]
                    ),
                };
                let val = match runtime.next_input() {
                    Some(val) => val,
                    None => return Err(EvalError::Finished),
                };
                runtime.memory[regnum] = val;
                Ok(pos + 1)
            }
        }
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_runtime_input() {
        let mut runtime = Runtime::new(2, vec![2.0, 3.0, 4.0]);
        assert_eq!(runtime.next_input(), Some(2.0));
        assert_eq!(runtime.next_input(), Some(3.0));
        assert_eq!(runtime.next_input(), Some(4.0));
        assert_eq!(runtime.next_input(), None);
    }

    #[test]
    fn test_stat_input() {
        let program: Vec<Token> = vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(0),
        ];
        let mut runtime = Runtime::new(2, vec![2.0]);
        assert_eq!(runtime.memory, vec![0.0, 0.0]);
        assert_eq!(runtime.input.len(), 1);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![2.0, 0.0]);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(matches!(res, Err(EvalError::Finished)));
    }

    #[test]
    fn test_stat_input_multiple() {
        let program: Vec<Token> = vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(0),
        ];
        let mut runtime = Runtime::new(2, vec![2.0, 3.0]);
        assert_eq!(runtime.memory, vec![0.0, 0.0]);
        assert_eq!(runtime.input.len(), 2);

        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![2.0, 0.0]);

        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![3.0, 0.0]);

        let res = eval_stat(&program, 0, &mut runtime);
        assert!(matches!(res, Err(EvalError::Finished)));
    }

    #[test]
    fn test_stat_input_second_register() {
        let program: Vec<Token> = vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(1)
        ];
        let mut runtime = Runtime::new(2, vec![4.0]);
        assert_eq!(runtime.memory, vec![0.0, 0.0]);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![0.0, 4.0]);
    }

    #[test]
    fn test_stat_output() {
        let program: Vec<Token> = vec![
            Token::Stat(Stat::OUTPUT),
            Token::Reg(0)
        ];
        let mut runtime = Runtime {
            memory: vec![2.0, 0.0],
            input: vec![],
            output: vec![],
            input_cursor: 0,
        };
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![2.0, 0.0]);
        assert_eq!(runtime.output, vec![2.0]);
    }

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

    #[test]
    fn test_exec_identity() {
        let program: Vec<Token> = vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(0),
            Token::Stat(Stat::OUTPUT),
            Token::Reg(0),
        ];
        let mut runtime = Runtime::new(2, vec![2.0]);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        let res = eval_stat(&program, 2, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.output, vec![2.0]);
    }
}