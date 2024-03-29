use super::common::*;

#[allow(unused)]
#[derive(Debug)]
pub enum EvalError {
    Finished,
    Syntax(usize, String),
    Semantic(String),
    MaxIteration
}

pub struct Runtime {
    memory: Vec<f32>,
    input: Vec<f32>,
    output: Vec<f32>,
    input_cursor: usize,
    max_iterations: usize,
}

impl Runtime {
    pub fn new(memsize: usize, input: Vec<f32>) -> Self {
        Runtime {
            memory: vec![0.0; memsize],
            input,
            output: Vec::new(),
            input_cursor: 0,
            max_iterations: 100
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
        Ok(pos) => {
            log::trace!("program ended with output {:?}", runtime.output);
            log::trace!("finished at pos {}/{}", pos, program.len() - 1);
            runtime.output
        }
        Err(EvalError::Finished) => {
            log::trace!(
                "terminated due to input end with output {:?}",
                runtime.output
            );
            runtime.output
        }
        Err(EvalError::MaxIteration) => {
            log::trace!(
                "terminated due reaching max iteration {:?}",
                runtime.output
            );
            runtime.output
        }
        Err(EvalError::Syntax(pos, reason)) => {
            log::error!("Invalid program: {program:?}");
            log::error!("Invalid syntax at {pos}: {reason}");
            panic!("Invalid syntax at {pos}: {reason}");
        }
        Err(EvalError::Semantic(reason)) => {
            log::error!("Invalid program: {program:?}");
            log::error!("Invalid program reason: {reason}");
            vec![f32::INFINITY]
        }
    };
}

// eval_block returns position after the last STAT. This means the cursor will point to ELSE or END tokens
fn eval_block(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    log::trace!("eval block {pos}");
    let mut pos = pos;
    loop {
        if pos >= program.len() || matches!(program[pos], Token::ELSE | Token::END) {
            log::trace!("returning from block, returning {pos}");
            return Ok(pos);
        }
        pos = eval_stat(program, pos, runtime)?;
    }
}

fn is_truthy(x: f32) -> bool {
    x != 0.0
}

fn handle_if_true(
    program: &Program,
    pos: usize,
    runtime: &mut Runtime,
) -> Result<usize, EvalError> {
    let end_or_else = eval_block(program, pos, runtime)?;
    match program[end_or_else] {
        Token::ELSE => {
            let else_part_end = get_node_end(program, end_or_else);
            log::trace!("got node end {else_part_end}");
            Ok(else_part_end)
        }
        Token::END => Ok(end_or_else + 1),
        _ => Err(EvalError::Syntax(
            end_or_else,
            "Expected END or ELSE".into(),
        )),
    }
}

fn handle_if_false(
    program: &Program,
    true_block_pos: usize,
    runtime: &mut Runtime,
    if_stat_pos: usize,
) -> Result<usize, EvalError> {
    let true_block_end = skip_block(program, true_block_pos);
    match program[true_block_end] {
        Token::ELSE => {
            let else_block_pos = true_block_end + 1;
            log::trace!("IF condition at {if_stat_pos} entered ELSE branch at {else_block_pos}");
            let endpos = eval_block(program, else_block_pos, runtime)?;
            Ok(endpos + 1)
        }
        Token::END => {
            log::trace!(
                "IF condition at {if_stat_pos} has no ELSE branch (reached END at {true_block_end}"
            );
            Ok(true_block_end + 1)
        }
        _ => unreachable!(),
    }
}

fn skip_block(program: &Program, pos: usize) -> usize {
    let mut level = 1;
    let mut cursor = pos;
    while cursor < program.len() && level > 0 {
        cursor += 1;
        match program[cursor] {
            Token::Stat(Stat::IF | Stat::WHILE) => level += 1,
            Token::END => level -= 1,
            Token::ELSE if level == 1 => level -= 1,
            _ => (),
        }
    }
    cursor
}

fn handle_while(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    let while_pos = pos;
    let expr_pos = pos + 1;
    let block_pos;
    let mut expr_val;
    (block_pos, expr_val) = eval_expr(program, expr_pos, runtime)?;
    let block_end_pos = skip_block(program, block_pos);
    let mut iteration = 0;

    while is_truthy(expr_val) {
        log::trace!("WHILE at {while_pos}: iteration {iteration}");
        eval_block(program, block_pos, runtime)?;
        (_, expr_val) = eval_expr(program, expr_pos, runtime)?;
        iteration += 1;
        if iteration >= runtime.max_iterations {
            return Err(EvalError::MaxIteration)
        }
    }

    match program[block_end_pos] {
        Token::END => Ok(block_end_pos + 1),
        _ => Err(EvalError::Syntax(block_end_pos, format!("Expected END after a WHILE started at {while_pos}. Got to {block_end_pos}")))
    }
}

fn eval_stat(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    log::trace!("eval stat {pos}");
    match program[pos] {
        Token::Stat(stat) => match stat {
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
                let (true_block_pos, condition_val) = eval_expr(program, pos + 1, runtime)?;
                if is_truthy(condition_val) {
                    log::trace!("IF condition at {pos} evaluated to TRUE");
                    handle_if_true(program, true_block_pos, runtime)
                } else {
                    log::trace!("IF condition at {pos} evaluated to FALSE");
                    handle_if_false(program, true_block_pos, runtime, pos)
                }
            }
            Stat::WHILE => handle_while(program, pos, runtime),
        },
        _ => panic!("called eval_stat on non-stat at {pos}"),
    }
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
            Expr::EQ => two_arg(equal, runtime),
            Expr::LT => two_arg(less_than, runtime),
            Expr::GT => two_arg(greater_than, runtime),
            Expr::OR => two_arg(or, runtime),
            Expr::AND => two_arg(and, runtime),
            Expr::NOT => one_arg(negation, runtime),
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
    fn equal(lhs: f32, rhs: f32) -> f32 {
        if lhs == rhs {
            1.0
        } else {
            0.0
        }
    }
    fn less_than(lhs: f32, rhs: f32) -> f32 {
        if lhs < rhs {
            1.0
        } else {
            0.0
        }
    }
    fn greater_than(lhs: f32, rhs: f32) -> f32 {
        if lhs > rhs {
            1.0
        } else {
            0.0
        }
    }
    fn or(lhs: f32, rhs: f32) -> f32 {
        if is_truthy(lhs) || is_truthy(rhs) {
            1.0
        } else {
            0.0
        }
    }
    fn and(lhs: f32, rhs: f32) -> f32 {
        if is_truthy(lhs) && is_truthy(rhs) {
            1.0
        } else {
            0.0
        }
    }
    fn negation(arg: f32) -> f32 {
        if is_truthy(arg) {
            0.0
        } else {
            1.0
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
            max_iterations: 100
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
        let mut runtime =  Runtime::new(3, vec![]);
        runtime.memory = data;
        let (pos, val) = eval_expr(&program, 0, &mut runtime).unwrap();
        assert_eq!(5, pos);
        assert_eq!(2.0, val);
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
