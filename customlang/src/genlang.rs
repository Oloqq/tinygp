#![allow(unused)]

use std::{str::FromStr, error::Error, fmt::format};

#[derive(Debug, PartialEq, Eq)]
enum Keyword {
    ADD,
}

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Kw(Keyword),
    Const(i32),
    Reg(usize)
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // keywords must not start with 'R'
        if s.starts_with("R") {
            return match s[1..].parse::<usize>() {
                Ok(num) => Ok(Opcode::Reg(num)),
                Err(_) => Err(format!("invalid register: {s}"))
            }
        }
        else if s.starts_with(|c: char| c.is_digit(10) || c == '-') {
            return match s.parse::<i32>() {
                Ok(num) => Ok(Opcode::Const(num)),
                Err(_) => Err(format!("invalid constant: {s}"))
            }
        }
        match s {
            "ADD" => Ok(Opcode::Kw(Keyword::ADD)),
            _ => Err(String::from(format!("unknown keyword: {s}")))
        }
    }
}

fn tokenize(program: &str) -> Vec<&str> {
    program
        .split([' ', '\t', '\r', '\n'])
        .filter(|t| t.len() > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    #[rustfmt::skip]
    fn test_tokenize() {
        assert_eq!(tokenize("ADD 2 3"), ["ADD", "2", "3"]);      // happy path
        assert_eq!(tokenize("  ADD  2 3  "), ["ADD", "2", "3"]); // multiple spaces
        assert_eq!(tokenize("  ADD\t2 3  "), ["ADD", "2", "3"]); // tab
        assert_eq!(tokenize("ADD\n  2 3"), ["ADD", "2", "3"]);   // unix newline
        assert_eq!(tokenize("ADD\r\n  2 3"), ["ADD", "2", "3"]); // windows newline
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
        assert_eq!(Opcode::from_str("10.0").unwrap_err(), "invalid constant: 10.0");
        assert_eq!(Opcode::from_str("1e2").unwrap_err(), "invalid constant: 1e2");

    }
}
