use rand_derive::Rand;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Rand, Serialize, Deserialize)]
pub enum Expr {
    ADD,
    SUB,
    MUL,
    DIV,
    SIN,
    COS,
    NUM(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Rand, Serialize, Deserialize)]
pub enum Stat {
    INPUT,
    OUTPUT,
    LOAD,
    IF,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Token {
    Expr(Expr),
    Stat(Stat),
    Reg(usize),
    ELSE,
    END,
}

pub const MAX_LEN: usize = 10000;

pub type Program = Vec<Token>;

impl Expr {
    pub fn argnum(&self) -> usize {
        match self {
            Expr::ADD => 2,
            Expr::SUB => 2,
            Expr::MUL => 2,
            Expr::DIV => 2,
            Expr::SIN => 1,
            Expr::COS => 1,
            Expr::NUM(_) => 0,
        }
    }
}

pub fn get_node_end(program: &Program, index: usize) -> usize {
    match program[index] {
        // no arguments
        Token::Reg(_) | Token::Expr(Expr::NUM(_)) => index + 1,
        // 1 argument
        Token::Stat(Stat::INPUT)
        | Token::Stat(Stat::OUTPUT)
        | Token::Expr(Expr::SIN)
        | Token::Expr(Expr::COS) => get_node_end(program, index + 1),
        // 2 arguments
        Token::Stat(Stat::LOAD)
        | Token::Expr(Expr::ADD)
        | Token::Expr(Expr::SUB)
        | Token::Expr(Expr::MUL)
        | Token::Expr(Expr::DIV) => {
            let arg1end = get_node_end(program, index + 1);
            get_node_end(program, arg1end)
        }
        // "parentheses counting"
        // TODO add WHILE
        Token::Stat(Stat::IF) | Token::ELSE => {
            let mut level = 1;
            let mut i = index;
            while i < program.len() && level > 0 {
                match program[i] {
                    // TODO add WHILE
                    Token::Stat(Stat::IF) => level += 1,
                    Token::END => level -= 1,
                    _ => (),
                }
                i += 1;
            }
            i
        }
        _ => todo!(),
    }
}

pub fn variant_eq(a: &Token, b: &Token) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_expression_end() {
        let program = vec![Token::Expr(Expr::ADD), Token::Reg(0), Token::Reg(0)];
        assert_eq!(get_node_end(&program, 0), 3);
        assert_eq!(get_node_end(&program, 1), 2);
        assert_eq!(get_node_end(&program, 2), 3);
        let program = vec![
            Token::Expr(Expr::ADD),
            Token::Expr(Expr::ADD),
            Token::Reg(0),
            Token::Reg(0),
            Token::Reg(0),
        ];
        assert_eq!(get_node_end(&program, 0), 5);
        assert_eq!(get_node_end(&program, 1), 4);
        assert_eq!(get_node_end(&program, 2), 3);
        assert_eq!(get_node_end(&program, 3), 4);
        assert_eq!(get_node_end(&program, 4), 5);
    }

    #[test]
    fn test_expression_end_if() {
        #[rustfmt::skip]
        let program = vec![
            Token::Stat(Stat::IF),
                Token::Expr(Expr::NUM(12.0)),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
            Token::END
        ];
        assert_eq!(get_node_end(&program, 0), 9);
    }

    #[test]
    fn test_expression_end_if_else() {
        #[rustfmt::skip]
        let program = vec![
            Token::Stat(Stat::IF),
                Token::Expr(Expr::NUM(12.0)),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
            Token::ELSE,
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
            Token::END
        ];
        assert_eq!(get_node_end(&program, 0), 12);
    }

    #[test]
    fn test_expression_end_nested_if() {
        #[rustfmt::skip]
        let program = vec![
            Token::Stat(Stat::IF),
                Token::Expr(Expr::NUM(12.0)),
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
                Token::Stat(Stat::IF),
                    Token::Expr(Expr::NUM(12.0)),
                    Token::Stat(Stat::OUTPUT), Token::Reg(0),
                    Token::Stat(Stat::OUTPUT), Token::Reg(0),
                    Token::Stat(Stat::OUTPUT), Token::Reg(0),
                Token::END,
                Token::Stat(Stat::OUTPUT), Token::Reg(0),
            Token::END
        ];
        assert_eq!(get_node_end(&program, 0), 16);
    }

    #[test]
    fn test_serialize() {
        let e = Expr::ADD;
        let s = serde_lexpr::to_string(&e).unwrap();
        println!("{s}");
        const INPUT: Token = Token::Stat(Stat::INPUT);
        const OUTPUT: Token = Token::Stat(Stat::OUTPUT);
        const LOAD: Token = Token::Stat(Stat::LOAD);
        use Token::Reg;
        let program = vec![
            INPUT,
            Reg(0),
            LOAD,
            Reg(1),
            Reg(0),
            OUTPUT,
            Reg(0),
            OUTPUT,
            Reg(1),
        ];
        let s = serde_lexpr::to_string(&program).unwrap();
        println!("{s}");
        let p2: Vec<Token> = serde_lexpr::from_str(&s).unwrap();
        println!("{program:?}");
        println!("{p2:?}");
        let s2 = serde_lexpr::to_string(&p2).unwrap();

        assert_eq!(s, s2);
    }
}
