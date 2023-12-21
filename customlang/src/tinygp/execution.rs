use super::common::*;

#[allow(unused)]
#[derive(Debug)]
pub enum EvalError {
    Finished,
    Syntax(usize, String),
    Semantic(String),
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
        if self.input_cursor < self.input.len() {
            let val = self.input[self.input_cursor];
            self.input_cursor += 1;
            Some(val)
        } else {
            None
        }
    }

    pub fn set_reg(&mut self, num: usize, val: f32) -> Result<(), EvalError> {
        if num > self.memory.len() {
            Err(EvalError::Semantic(format!(
                "Tried to set memory[{num}], when length is {}",
                self.memory.len()
            )))
        } else {
            self.memory[num] = val;
            Ok(())
        }
    }

    pub fn read_reg(&self, num: usize) -> Result<f32, EvalError> {
        if num > self.memory.len() {
            Err(EvalError::Semantic(format!(
                "Tried to read memory[{num}], when length is {}",
                self.memory.len()
            )))
        } else {
            Ok(self.memory[num].clone())
        }
    }
}

pub fn execute(program: &Program, runtime: Runtime) -> Vec<f32> {
    log::trace!("executing {:?}", program);
    let mut runtime = runtime;
    return match eval_block(program, 0, &mut runtime) {
        Ok(_) | Err(EvalError::Finished) => {
            log::trace!("finished with output {:?}", runtime.output);
            runtime.output
        }
        Err(_) => {
            log::error!("Invalid program: {program:?}");
            vec![f32::INFINITY]
        }
    };
}

fn eval_block(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    log::trace!("eval block {pos}");
    let mut pos = pos;
    loop {
        if pos >= program.len() || matches!(program[pos], Token::ELSE | Token::END) {
            return Ok(pos);
        }
        pos = eval_stat(program, pos, runtime)?;
    }
}

fn is_truthy(x: f32) -> bool {
    x != 0.0
}

fn eval_stat(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    log::trace!("eval stat {pos}");
    if let Token::Stat(stat) = program[pos] {
        return match stat {
            Stat::OUTPUT => {
                let (newpos, val) = eval_expr(program, pos + 1, runtime)?;
                runtime.output.push(val);
                Ok(newpos)
            }
            Stat::INPUT => {
                let destination = match program[pos + 1] {
                    Token::Reg(num) => num,
                    _ => return Err(EvalError::Syntax(pos + 1, "Expected REG".into())),
                };
                let val = match runtime.next_input() {
                    Some(val) => val,
                    None => return Err(EvalError::Finished),
                };
                runtime.set_reg(destination, val)?;
                Ok(pos + 2)
            }
            Stat::LOAD => {
                let destination = match program[pos + 1] {
                    Token::Reg(num) => num,
                    _ => return Err(EvalError::Syntax(pos + 1, "Expected REG".into())),
                };
                let (newpos, val) = eval_expr(program, pos + 2, runtime)?;
                runtime.set_reg(destination, val)?;
                Ok(newpos)
            }
            Stat::IF => {
                let (blockpos, condition_val) = eval_expr(program, pos + 1, runtime)?;
                if is_truthy(condition_val) {
                    log::trace!("IF condition at {pos} entered branch TRUE");
                    let newpos = eval_block(program, blockpos, runtime)?;
                    if matches!(program[newpos], Token::ELSE) {
                        let else_part_end = get_node_end(program, newpos);
                        Ok(else_part_end + 1)
                    } else if matches!(program[newpos], Token::END) {
                        Ok(newpos + 1)
                    } else {
                        unreachable!()
                    }
                } else {
                    log::trace!("IF condition at {pos} entered branch FALSE");
                    let mut level = 1;
                    let mut elsepos = blockpos;
                    while elsepos < program.len() && level > 0 {
                        match program[elsepos] {
                            // TODO add WHILE
                            Token::Stat(Stat::IF) => level += 1,
                            Token::ELSE if level == 1 => {
                                elsepos += 1;
                                break
                            }
                            Token::END => level -= 1,
                            _ => (),
                        }
                        elsepos += 1;
                    }
                    eval_block(program, elsepos, runtime)
                }
            }
        };
    }
    panic!("called eval_stat on non-stat");
}

fn eval_expr(
    program: &Program,
    pos: usize,
    runtime: &mut Runtime,
) -> Result<(usize, f32), EvalError> {
    let opcode = program[pos];

    let one_arg = |func: fn(f32) -> f32, runtime: &mut Runtime| {
        let (pos, arg) = eval_expr(program, pos + 1, runtime)?;
        Ok((pos, func(arg)))
    };

    let two_arg = |func: fn(f32, f32) -> f32, runtime: &mut Runtime| {
        let (pos, lhs) = eval_expr(program, pos + 1, runtime)?;
        let (pos, rhs) = eval_expr(program, pos, runtime)?;
        Ok((pos, func(lhs, rhs)))
    };

    return match opcode {
        Token::Expr(func) => match func {
            Expr::NUM(val) => Ok((pos + 1, val)),
            Expr::ADD => two_arg(add, runtime),
            Expr::SUB => two_arg(sub, runtime),
            Expr::MUL => two_arg(mul, runtime),
            Expr::DIV => two_arg(protected_div, runtime),
            Expr::SIN => one_arg(f32::sin, runtime),
            Expr::COS => one_arg(f32::cos, runtime),
        },
        Token::Reg(num) => Ok((pos + 1, runtime.read_reg(num)?)),
        _ => unreachable!("called eval_expr on non-expr: {opcode:?}"),
    };

    fn add(lhs: f32, rhs: f32) -> f32 {
        lhs + rhs
    }
    fn sub(lhs: f32, rhs: f32) -> f32 {
        lhs - rhs
    }
    fn mul(lhs: f32, rhs: f32) -> f32 {
        lhs * rhs
    }
    fn protected_div(lhs: f32, rhs: f32) -> f32 {
        if rhs.abs() <= 0.001 {
            lhs
        } else {
            lhs / rhs
        }
    }
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
        let program: Vec<Token> = vec![Token::Stat(Stat::INPUT), Token::Reg(0)];
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
        let program: Vec<Token> = vec![Token::Stat(Stat::INPUT), Token::Reg(0)];
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
        let program: Vec<Token> = vec![Token::Stat(Stat::INPUT), Token::Reg(1)];
        let mut runtime = Runtime::new(2, vec![4.0]);
        assert_eq!(runtime.memory, vec![0.0, 0.0]);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![0.0, 4.0]);
    }

    #[test]
    fn test_stat_output() {
        let program: Vec<Token> = vec![Token::Stat(Stat::OUTPUT), Token::Reg(0)];
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
    #[ignore]
    fn test_expression() {
        // let program: Vec<Token> = vec![
        //     Token::Expr(Expr::ADD),
        //     Token::Reg(0),
        //     Token::Expr(Expr::DIV),
        //     Token::Reg(1),
        //     Token::Reg(1),
        // ];
        // let data = vec![1.0, -2.0];
        // assert_eq!(2.0, eval_expr(&program, &data, &mut 0));

        // let program: Vec<Token> = vec![
        //     Token::Expr(Expr::SUB),
        //     Token::Reg(0),
        //     Token::Expr(Expr::DIV),
        //     Token::Reg(1),
        //     Token::Reg(2),
        // ];
        // assert_eq!(
        //     0.8776571,
        //     eval_expr(
        //         &program,
        //         &vec![0.0, -4.025456902691228, 4.58659426408455],
        //         &mut 0
        //     )
        // );
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
