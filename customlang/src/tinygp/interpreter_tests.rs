use super::{
    common::*,
    execution::{execute, Runtime},
};

const INPUT: Token = Token::Stat(Stat::INPUT);
const OUTPUT: Token = Token::Stat(Stat::OUTPUT);
const LOAD: Token = Token::Stat(Stat::LOAD);
use Token::Reg;

use pretty_assertions::assert_eq;

fn run_cases(program: &Program, memsize: usize, cases: Vec<(Vec<f32>, Vec<f32>)>) {
    let _ = env_logger::builder().is_test(true).try_init();
    for (i, (input, expected_output)) in cases.into_iter().enumerate() {
        let runtime = Runtime::new(memsize, input);
        let output = execute(&program, runtime);
        println!("\nCase {i}");
        assert_eq!(output, expected_output);
    }
}

#[test]
fn test_identity() {
    let memsize = 3;
    let program = vec![INPUT, Token::Reg(0), OUTPUT, Token::Reg(0)];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![(vec![1.0], vec![1.0]), (vec![2.0], vec![2.0])];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_register() {
    let memsize = 2;
    let program = vec![
        INPUT, Reg(0),
        LOAD, Reg(1), Reg(0),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
    ];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![
        (vec![1.0], vec![1.0, 1.0]),
        (vec![2.0], vec![2.0, 2.0])
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_expr_add() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::ADD), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![
        (vec![1.0, 2.0], vec![1.0, 2.0, 3.0]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_expr_sub() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::SUB), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![
        (vec![1.0, 2.0], vec![1.0, 2.0, -1.0]),
    ];
    run_cases(&program, memsize, cases);
}

#[test]
#[rustfmt::skip]
fn test_load_expr_mul() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::MUL), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![
        (vec![2.0, 2.0], vec![2.0, 2.0, 4.0]),
    ];
    run_cases(&program, memsize, cases);
}


#[test]
#[rustfmt::skip]
fn test_load_expr_protected_div() {
    let memsize = 3;
    let program = vec![
        INPUT, Reg(0),
        INPUT, Reg(1),
        LOAD, Reg(2),
            Token::Expr(Expr::DIV), Reg(0), Reg(1),
        OUTPUT, Reg(0),
        OUTPUT, Reg(1),
        OUTPUT, Reg(2),
    ];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![
        (vec![1.0, 2.0], vec![1.0, 2.0, 0.5]),
        (vec![1.0, 0.01], vec![1.0, 0.01, 100.0]),
        (vec![1.0, 0.0001], vec![1.0, 0.0001, 1.0]),
    ];
    run_cases(&program, memsize, cases);
}
