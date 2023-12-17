
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Token {
    Const(i32),
    Reg(usize),
    ADD,
    LOAD,
    OUTPUT,
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // keywords must not start with 'R'
        if s.starts_with("R") {
            return match s[1..].parse::<usize>() {
                Ok(num) => Ok(Token::Reg(num)),
                Err(_) => Err(format!("invalid register: {s}")),
            };
        } else if s.starts_with(|c: char| c.is_digit(10) || c == '-') {
            return match s.parse::<i32>() {
                Ok(num) => Ok(Token::Const(num)),
                Err(_) => Err(format!("invalid constant: {s}")),
            };
        }
        match s {
            "ADD" => Ok(Token::ADD),
            "LOAD" => Ok(Token::LOAD),
            "OUTPUT" => Ok(Token::OUTPUT),
            _ => Err(String::from(format!("unknown keyword: {s}"))),
        }
    }
}

pub fn tokenize(program: &str) -> Vec<Token> {
    program
        .split([' ', '\t', '\r', '\n'])
        .filter(|t| t.len() > 0)
        .map(|t| Token::from_str(t).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    #[test]
    fn test_tokenize() {
        let program = [
            Token::ADD,
            Token::Const(2),
            Token::Const(3),
        ];
        assert_eq!(tokenize("ADD 2 3"), program);
        assert_eq!(tokenize("  ADD  2 3  "), program);
        assert_eq!(tokenize("  ADD\t2 3  "), program);
        assert_eq!(tokenize("ADD\n  2 3"), program);
        assert_eq!(tokenize("ADD\r\n  2 3"), program);
    }

    #[test]
    fn test_opcode_from_string() {
        // registers
        assert_eq!(Token::from_str("R123").unwrap(), Token::Reg(123));
        assert_eq!(Token::from_str("R0").unwrap(), Token::Reg(0));
        assert_eq!(Token::from_str("R").unwrap_err(), "invalid register: R");
        assert_eq!(Token::from_str("RR").unwrap_err(), "invalid register: RR");
        // constants
        assert_eq!(Token::from_str("0").unwrap(), Token::Const(0));
        assert_eq!(Token::from_str("999").unwrap(), Token::Const(999));
        assert_eq!(Token::from_str("-999").unwrap(), Token::Const(-999));
        assert_eq!(
            Token::from_str("10.0").unwrap_err(),
            "invalid constant: 10.0"
        );
        assert_eq!(
            Token::from_str("1e2").unwrap_err(),
            "invalid constant: 1e2"
        );
        // keywords
        assert_eq!(Token::from_str("ADD").unwrap(), Token::ADD);
        for kw in Token::iter() {
            if kw == Token::Const(0) || kw == Token::Reg(0) {
                continue;
            }
            let s = format!("{:?}", kw);
            assert_eq!(
                Token::from_str(s.as_str()).expect(format!("you forgot to implement from_str for keyword {:?}", kw).as_str()),
                kw
            );
        }
        assert_eq!(
            Token::from_str("bruh").unwrap_err(),
            "unknown keyword: bruh"
        );
    }
}
