use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
pub enum Funcs {
    Start = 110, // number important for serialization, TODO after the course calculate the index dynamically based on number of variables and const numbers
    ADD,
    SUB,
    MUL,
    DIV,
    SIN,
    COS,
    INPUT,
    OUTPUT,
    End, // need to generate ranges, TODO after the course get rid of it along with Funcs::Start
}

#[allow(unused)]
pub const CONST_NUM: usize = 0;
pub const TOKEN_STAT: [Funcs; 2] = [Funcs::OUTPUT, Funcs::INPUT];

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Kw(Funcs),
    Reg(usize),
}

pub const MAX_LEN: usize = 10000;

pub type Program = Vec<Token>;

pub fn get_node_end(program: &Program, start: usize) -> usize {
    let arg1 = |index| -> usize { get_node_end(program, index) };
    let arg2 = |index| -> usize {
        let arg1end = get_node_end(program, index);
        get_node_end(program, arg1end)
    };

    match program[start] {
        Token::Reg(_) => start + 1,
        Token::Kw(k) => match k {
            Funcs::Start => unreachable!(),
            Funcs::ADD => arg2(start + 1),
            Funcs::SUB => arg2(start + 1),
            Funcs::MUL => arg2(start + 1),
            Funcs::DIV => arg2(start + 1),
            Funcs::SIN => arg1(start + 1),
            Funcs::COS => arg1(start + 1),
            Funcs::INPUT => arg1(start + 1),
            Funcs::OUTPUT => arg1(start + 1),
            Funcs::End => unreachable!(),
            // get_node_end(program, get_node_end(program, start + 1)),
        },
    }
}