#![allow(dead_code, unused_variables)]

use rand::prelude::*;
use rand::SeedableRng;
use std::error::Error;
use std::fmt::Display;
use std::fs;

type Berror = Box<dyn Error>;
type Case = Vec<f32>;

const ADD: usize = 110;
const SUB: usize = 111;
const MUL: usize = 112;
const DIV: usize = 113;
const FSET_START: usize = ADD;
const FSET_END: usize = DIV;

type Opcode = usize;
type Program = Vec<Opcode>;

struct Params {
    seed: u64,
    min_random: f32,
    max_random: f32,
    varnumber: i32,
    const_numbers: i32,
    max_len: usize,
    popsize: usize,
    depth: usize,
    crossover_prob: f32,
    pmut_per_node: f32,
    tournament_size: usize,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            min_random: Default::default(),
            max_random: Default::default(),
            varnumber: Default::default(),
            const_numbers: Default::default(),
            max_len: 10000,
            popsize: 100000,
            depth: 5,
            crossover_prob: 0.9,
            pmut_per_node: 0.05,
            tournament_size: 2,
        }
    }
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "SEED={}\nMAX_LEN={})
POPSIZE={}
DEPTH={})
CROSSOVER_PROB={})
PMUT_PER_NODE={})
MIN_RANDOM={})
MAX_RANDOM={})
TSIZE={})
----------------------------------\n",
                self.seed,
                self.max_len,
                self.popsize,
                self.depth,
                self.crossover_prob,
                self.pmut_per_node,
                self.min_random,
                self.max_random,
                self.tournament_size
            )
            .as_str(),
        )
    }
}

fn execute(program: &Program, variables: &Vec<f32>, cursor: &mut usize) -> f32 {
    let opcode = program[*cursor];
    *cursor += 1;

    assert!(opcode <= FSET_END);
    return match opcode {
        ADD => execute(program, variables, cursor) + execute(program, variables, cursor),
        SUB => execute(program, variables, cursor) - execute(program, variables, cursor),
        MUL => execute(program, variables, cursor) * execute(program, variables, cursor),
        DIV => {
            let numerator = execute(program, variables, cursor);
            let denominator = execute(program, variables, cursor);
            if denominator.abs() <= 0.001 {
                numerator
            } else {
                numerator / denominator
            }
        }
        _ => variables[opcode],
    };
}

fn read_problem(data: String) -> Result<(Params, Vec<Case>), Berror> {
    let lines: Vec<&str> = data.split('\n').collect();
    // println!("line {:?}", lines);
    let header: Vec<&str> = lines[0].trim().split([' ', '\t']).collect();
    let varnumber: i32 = header[0].parse()?;
    let random_number: i32 = header[1].parse()?;
    let min_random: f32 = header[2].parse()?;
    let max_random: f32 = header[3].parse()?;
    let num_cases: usize = header[4].parse()?;

    let mut cases: Vec<Case> = Vec::with_capacity(num_cases);
    for i in 0..num_cases {
        let tokens: Vec<&str> = lines[i + 1]
            .trim()
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
            const_numbers: random_number,
            ..Default::default()
        },
        cases,
    ))
}

struct TinyGP {
    rand: StdRng,
    params: Params,
    cases: Vec<Case>,
    generation: i32,
}

impl TinyGP {
    fn new(params: Params, cases: Vec<Case>) -> TinyGP {
        TinyGP {
            rand: StdRng::seed_from_u64(params.seed),
            params,
            cases,
            generation: 0,
        }
    }

    pub fn from_problem(filename: &str) -> Result<TinyGP, Berror> {
        let content = fs::read_to_string(filename)?;
        println!("{content}");
        let (params, cases) = read_problem(content)?;
        Ok(TinyGP::new(params, cases))
    }

    pub fn evolve(&mut self, generations: usize) {
        println!(
            "-- TINY GP (Rust version) --\nGENERATIONS={}\n{}",
            generations,
            self.params
        );
        self.stats()
    }

    fn stats(&mut self) {
        // let best = self.rand.gen_range(0, self.params.popsize);
        // let mut node_count = 0;
    }
}

fn main() {
    let seed: Option<i32> = Some(3);
    let filename = "../linear.dat";

    let mut tgp = TinyGP::from_problem(filename).unwrap();
    tgp.evolve(100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_params() {
        let (param, cases) = match read_problem(
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

    #[test]
    fn test_execute() {
        let program = vec![110, 0, 113, 1, 1];
        assert_eq!(2.0, execute(&program, &vec![1.0, -2.0], &mut 0));

        let program = vec![111, 0, 113, 1, 2];
        assert_eq!(
            0.8776571,
            execute(
                &program,
                &vec![0.0, -4.025456902691228, 4.58659426408455],
                &mut 0
            )
        );
    }
}
