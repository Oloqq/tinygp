use rand::prelude::*;
use rand::SeedableRng;
use std::error::Error;
use std::fs;

use crate::params;
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
        let mut rand = StdRng::seed_from_u64(params.seed);
        let (population, fitness) = random_population(&params, &cases, &mut rand);
        TinyGP {
            rand,
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
        let mut generations = generations;
        let mut best_fitness = self.stats();
        while best_fitness < self.params.acceptable_error && generations > 0 {
            generations -= 1;
            self.evolve_generation();
            best_fitness = self.stats();
        }

        if best_fitness > self.params.acceptable_error {
            println!("PROBLEM SOLVED");
        } else {
            println!("PROBLEM UNSOLVED");
        }
    }

    fn evolve_generation(&mut self) {
        for _ in 0..self.params.popsize {
            let child: Program = if self.rand.gen_bool(self.params.crossover_prob as f64) {
                let father_id =
                    tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                let mother_id =
                    tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                todo!() //crossover(self.population[father_id], self.population[mother_id]);
            } else {
                let parent = tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                // mutation()
                todo!();
            };
        }
    }

    fn stats(&mut self) -> f32 {
        // let best = self.rand.gen_range(0, self.params.popsize);
        // let mut node_count = 0;
        0.0
    }
}

fn tournament(fitness: &Vec<f32>, tournament_size: usize, rand: &mut StdRng) -> usize {
    let mut best = rand.gen_range(0, fitness.len());
    let mut best_fitness = fitness[best];

    for _ in 0..tournament_size {
        let competitor = rand.gen_range(0, fitness.len());
        if fitness[competitor] > best_fitness {
            best_fitness = fitness[competitor];
            best = competitor;
        }
    }
    best
}

fn negative_tournament(fitness: &Vec<f32>, tournament_size: usize, rand: &mut StdRng) -> usize {
    let mut worst = rand.gen_range(0, fitness.len());
    let mut worst_fitness = fitness[worst];

    for _ in 0..tournament_size {
        let competitor = rand.gen_range(0, fitness.len());
        if fitness[competitor] < worst_fitness {
            worst_fitness = fitness[competitor];
            worst = competitor;
        }
    }
    worst
}

// choose non terminal or terminal until depth is reached, then choose only terminals
fn grow(program: &mut Program, depth: usize, params: &Params, rand: &mut StdRng) {
    if depth > 0 && rand.gen_bool(0.5) {
        let operation = rand.gen_range(FSET_START, FSET_END + 1);
        assert!([ADD, SUB, MUL, DIV].contains(&operation));
        program.push(operation);
        // grow operands
        grow(program, depth - 1, params, rand);
        grow(program, depth - 1, params, rand);
    } else {
        let terminal: usize = rand.gen_range(0, params.varnumber + params.const_numbers) as usize;
        program.push(terminal);
    }
}

fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(2 * params.depth);
    grow(&mut program, params.depth, params, rand);
    program
}

fn fitness_func(program: &Program, cases: &Vec<Case>) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let output = execute(program, inputs, &mut 0);
        let error = (output - targets[0]).abs();
        acc - error
    })
}

fn random_population(
    params: &Params,
    cases: &Vec<Case>,
    rand: &mut StdRng,
) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(params.popsize);
    let mut fitness = Vec::with_capacity(params.popsize);

    for i in 0..params.popsize {
        population[i] = create_random_indiv(params, rand);
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

    fn mock_params() -> Params {
        Params {
            seed: 1,
            min_random: -1.0,
            max_random: 1.0,
            varnumber: 2,
            const_numbers: 2,
            popsize: 10,
            depth: 3,
            crossover_prob: 0.9,
            pmut_per_node: 0.1,
            tournament_size: 2,
            acceptable_error: -1e-5
        }
    }

    #[test]
    fn test_grow_depth_0() {
        let mut program = Vec::new();
        let mut rand: StdRng = StdRng::seed_from_u64(1);
        grow(&mut program, 0, &mock_params(), &mut rand);
        assert!(program.len() == 1)
    }
}
