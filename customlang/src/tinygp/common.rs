#![allow(unused)]

use num_derive::FromPrimitive;
use rand_derive::Rand;

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Rand)]
pub enum Expr {
    ADD,
    SUB,
    MUL,
    DIV,
    SIN,
    COS,
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Rand)]
pub enum Stat {
    INPUT,
    OUTPUT,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Expr(Expr),
    Stat(Stat),
    Reg(usize),
}

pub const MAX_LEN: usize = 10000;

pub type Program = Vec<Token>;

pub fn get_node_end(program: &Program, index: usize) -> usize {
    match program[index] {
        Token::Reg(_) => index + 1,
        Token::Stat(Stat::INPUT)
        | Token::Stat(Stat::OUTPUT)
        | Token::Expr(Expr::SIN)
        | Token::Expr(Expr::COS) => get_node_end(program, index + 1),
        Token::Expr(Expr::ADD)
        | Token::Expr(Expr::SUB)
        | Token::Expr(Expr::MUL)
        | Token::Expr(Expr::DIV) => {
            let arg1end = get_node_end(program, index + 1);
            get_node_end(program, arg1end)
        },
    }
}

pub fn variant_eq(a: &Token, b: &Token) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
