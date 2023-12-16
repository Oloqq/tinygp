#![allow(unused)]

use std::{error::Error, fmt::format, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, EnumIter)]
enum Keyword {
    ADD,
}

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Kw(Keyword),
    Const(i32),
    Reg(usize),
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // keywords must not start with 'R'
        if s.starts_with("R") {
            return match s[1..].parse::<usize>() {
                Ok(num) => Ok(Opcode::Reg(num)),
                Err(_) => Err(format!("invalid register: {s}")),
            };
        } else if s.starts_with(|c: char| c.is_digit(10) || c == '-') {
            return match s.parse::<i32>() {
                Ok(num) => Ok(Opcode::Const(num)),
                Err(_) => Err(format!("invalid constant: {s}")),
            };
        }
        match s {
            "ADD" => Ok(Opcode::Kw(Keyword::ADD)),
            _ => Err(String::from(format!("unknown keyword: {s}"))),
        }
    }
}

fn tokenize(program: &str) -> Vec<Opcode> {
    program
        .split([' ', '\t', '\r', '\n'])
        .filter(|t| t.len() > 0)
        .map(|t| Opcode::from_str(t).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_tokenize() {
        let program = [
            Opcode::Kw(Keyword::ADD),
            Opcode::Const(2),
            Opcode::Const(3),
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
        assert_eq!(Opcode::from_str("R123").unwrap(), Opcode::Reg(123));
        assert_eq!(Opcode::from_str("R0").unwrap(), Opcode::Reg(0));
        assert_eq!(Opcode::from_str("R").unwrap_err(), "invalid register: R");
        assert_eq!(Opcode::from_str("RR").unwrap_err(), "invalid register: RR");
        // constants
        assert_eq!(Opcode::from_str("0").unwrap(), Opcode::Const(0));
        assert_eq!(Opcode::from_str("999").unwrap(), Opcode::Const(999));
        assert_eq!(Opcode::from_str("-999").unwrap(), Opcode::Const(-999));
        assert_eq!(
            Opcode::from_str("10.0").unwrap_err(),
            "invalid constant: 10.0"
        );
        assert_eq!(
            Opcode::from_str("1e2").unwrap_err(),
            "invalid constant: 1e2"
        );
        // keywords
        assert_eq!(Opcode::from_str("ADD").unwrap(), Opcode::Kw(Keyword::ADD));
        for kw in Keyword::iter() {
            let s = format!("{:?}", kw);
            assert_eq!(
                Opcode::from_str(s.as_str()).expect(format!("you forgot to implement from_str for {:?}", kw).as_str()),
                Opcode::Kw(kw)
            );
        }
        assert_eq!(
            Opcode::from_str("bruh").unwrap_err(),
            "unknown keyword: bruh"
        );
    }
}
