#![allow(dead_code, unused_variables)]

use std::error::Error;
// use std::fs;

type Berror = Box<dyn Error>;
type Case = Vec<f32>;

struct Params {
    seed: i32,
    min_random: f32,
    max_random: f32,
    varnumber: i32,
    random_number: i32, // what is this
}

struct TinyGP {}

impl TinyGP {
    pub fn with_params() -> TinyGP {
        todo!();
    }

    fn read_problem(data: String) -> Result<(Params, Vec<Case>), Berror> {
        let lines: Vec<&str> = data.split('\n').collect();
        // println!("line {:?}", lines);
        let header: Vec<&str> = lines[0].split([' ', '\t']).collect();
        let varnumber: i32 = header[0].parse()?;
        let random_number: i32 = header[1].parse()?;
        let min_random: f32 = header[2].parse()?;
        let max_random: f32 = header[3].parse()?;
        let num_cases: usize = header[4].parse()?;

        let mut cases: Vec<Case> = Vec::with_capacity(num_cases);
        for i in 0..num_cases {
            let tokens: Vec<&str> = lines[i + 1]
                .split([' ', '\t'])
                .filter(|t| !t.is_empty())
                .collect();
            let case: Case = tokens.iter().map(|t| t.parse().unwrap()).collect();
            cases.push(case);
        }

        Ok((
            Params {
                seed: 5,
                min_random,
                max_random,
                varnumber,
                random_number
            },
            cases,
        ))
    }

    pub fn from_problem(filename: &str) -> Result<TinyGP, Berror> {
        todo!();
    }
}

fn main() {
    let seed: Option<i32> = Some(3);
    let filename = "../linear.dat";

    let tgp = TinyGP::from_problem(filename);
}

#[cfg(test)]
mod tests {
    use crate::TinyGP;

    #[test]
    fn test_read_params() {
        let (param, cases) = match TinyGP::read_problem(
            "1 100 -5 5 10
0 1
1	2
2	3
3	4
4   5
5   6
6   7
7   8
8   9
9   10"
                .to_owned(),
        ) {
            Ok(p) => p,
            Err(_) => panic!("Read problem failed"),
        };

        assert_eq!(param.seed, 5);
        assert_eq!(param.min_random, -5.0);
        assert_eq!(param.max_random, 5.0);

        assert_eq!(cases.len(), 10);
    }
}
