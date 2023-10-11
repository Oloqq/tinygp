use rand::prelude::*;
use rand::SeedableRng;
use std::error::Error;
use std::fs;

use crate::params::Case;
use crate::params::Params;

const ADD: usize = 110;
const SUB: usize = 111;
const MUL: usize = 112;
const DIV: usize = 113;
const FSET_START: usize = ADD;
const FSET_END: usize = DIV;

pub type Opcode = usize;
pub type Program = Vec<Opcode>;

pub struct TinyGP {
    rand: StdRng,
    params: Params,
    cases: Vec<Case>,
    generation: i32,
    population: Vec<Program>,
    fitness: Vec<f32>,
    variables: Vec<f32>,
}

impl TinyGP {
    fn new(params: Params, cases: Vec<Case>) -> TinyGP {
        let (population, fitness) = random_population(&params, &cases);
        TinyGP {
            rand: StdRng::seed_from_u64(params.seed),
            fitness,
            population,
            params,
            cases,
            generation: 0,
            variables: Vec::with_capacity(FSET_START),
        }
    }

    fn init_population(&self) -> Vec<Program> {
        vec![]
    }

    pub fn from_problem(filename: &str) -> Result<TinyGP, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        println!("{content}");
        let (params, cases) = Params::from_string(content)?;
        Ok(TinyGP::new(params, cases))
    }

    pub fn evolve(&mut self, generations: usize) {
        println!(
            "-- TINY GP (Rust version) --\nGENERATIONS={}\n{}",
            generations, self.params
        );
        self.stats()
    }

    fn stats(&mut self) {
        // let best = self.rand.gen_range(0, self.params.popsize);
        // let mut node_count = 0;
    }
}

fn create_random_indiv(depth: usize) -> Program {
    todo!();
}

fn fitness_func(program: &Program, cases: &Vec<Case>) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let output = execute(program, inputs, &mut 0);
        let error = (output - targets[0]).abs();
        acc - error
    })
}

fn random_population(p: &Params, cases: &Vec<Case>) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(p.popsize);
    let mut fitness = Vec::with_capacity(p.popsize);

    for i in 0..p.popsize {
        population[i] = create_random_indiv(p.depth);
        fitness[i] = fitness_func(&population[i], cases);
    }

    return (population, fitness);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let program = vec![ADD, 0, DIV, 1, 1];
        let data = vec![1.0, -2.0];
        assert_eq!(2.0, execute(&program, &data, &mut 0));

        let program = vec![SUB, 0, DIV, 1, 2];
        assert_eq!(
            0.8776571,
            execute(
                &program,
                &vec![0.0, -4.025456902691228, 4.58659426408455],
                &mut 0
            )
        );
    }

    #[test]
    fn test_fitness() {
        let program = vec![ADD, 0, DIV, 1, 1];

        let cases: Vec<Case> = vec![(vec![1.0, 2.0], vec![2.0])];
        let result = fitness_func(&program, &cases);
        assert_eq!(result, 0.0);

        let cases: Vec<Case> = vec![(vec![1.0, 2.0], vec![0.0]), (vec![1.0, 2.0], vec![0.0])];
        let result = fitness_func(&program, &cases);
        assert_eq!(result, -4.0);
    }
}
