pub mod common;
mod evolution;
mod execution;
mod growing;
pub mod fitness_funcs;

#[cfg(test)]
mod evolution_tests;
#[cfg(test)]
mod interpreter_tests;

use crate::params::Case;
use crate::params::Params;
use common::*;
use evolution::*;
// use execution::*;
use growing::*;

use rand::prelude::*;
use rand::SeedableRng;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::io::Write;

use self::fitness_funcs::*;

pub struct TinyGP {
    rand: StdRng,
    params: Params,
    cases: Vec<Case>,
    generation: i32,
    population: Vec<Program>,
    fitness: Vec<f32>,
    writer: RefCell<Box<dyn Write>>,
}

impl TinyGP {
    pub fn new(
        mut params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
        fitness_func: FitnessFunc
    ) -> TinyGP {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        let mut rand = StdRng::seed_from_u64(seed);
        params.seed = seed;
        writeln!(writer.borrow_mut(), "Creating population").unwrap();
        let (population, fitness) = random_population(&params, &cases, &mut rand, fitness_func);
        TinyGP {
            rand,
            fitness,
            population,
            params,
            cases,
            generation: 0,
            writer: writer.into(),
        }
    }

    #[allow(unused)]
    pub fn from_problem(
        filename: &str,
        seed: Option<u64>,
        writer: Box<dyn Write>,
    ) -> Result<TinyGP, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        let writer = RefCell::new(writer);
        writeln!(*writer.borrow_mut(), "{content}").unwrap();
        let (params, cases) = Params::from_string(content)?;
        writeln!(*writer.borrow_mut(), "{}", cases.len()).unwrap();
        let tmp_default_fitness_func: FitnessFunc = *FITNESS_FUNCS.get("diff_first".into()).unwrap();
        Ok(TinyGP::new(params, cases, seed, writer, tmp_default_fitness_func))
    }

    pub fn evolve(&mut self, generations: usize, fitness_func: FitnessFunc) -> (Program, f32) {
        writeln!(
            self.writer.borrow_mut(),
            "-- TINY GP (Rust version) --\nGENERATIONS={}\n{}",
            generations,
            self.params
        )
        .unwrap();
        let mut generations = generations;
        let (mut best_fitness, mut best_id) = self.stats();
        while best_fitness < self.params.acceptable_error && generations > 0 {
            generations -= 1;
            self.evolve_generation(fitness_func);
            (best_fitness, best_id) = self.stats();
            self.writer.borrow_mut().flush().unwrap();
        }

        if best_fitness >= self.params.acceptable_error {
            writeln!(self.writer.borrow_mut(), "PROBLEM SOLVED").unwrap();
            fs::write("solution.txt", format!("{:?}", self.population[best_id])).unwrap();
        } else {
            writeln!(self.writer.borrow_mut(), "PROBLEM UNSOLVED").unwrap();
        }
        self.writer.borrow_mut().flush().unwrap();
        (self.population[best_id].clone(), best_fitness)
    }

    fn evolve_generation(&mut self, fitness_func: FitnessFunc) {
        for _ in 0..self.params.popsize {
            let child_program: Program;
            if self.rand.gen_bool(self.params.p_crossover as f64) {
                let father_id =
                    tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                let mother_id =
                    tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                let father = &self.population[father_id];
                let mother = &self.population[mother_id];
                child_program = crossover(father, mother, &mut self.rand);
            } else {
                let parent_id =
                    tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                let parent = &self.population[parent_id];
                child_program = mutation(parent, &self.params, &mut self.rand);
            };
            let child_index =
                negative_tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
            self.fitness[child_index] = run_and_rank(&child_program, &self.params, &self.cases, fitness_func);
            self.population[child_index] = child_program;
        }
        self.generation += 1;
    }

    fn stats(&mut self) -> (f32, usize) {
        let mut best = 0;
        let mut node_count = 0;
        let mut best_fitness = f32::MIN;
        let mut avg_fitness = 0.0;
        let popsize = self.population.len();

        for i in 0..popsize {
            node_count += self.population[i].len();
            avg_fitness += self.fitness[i];
            if self.fitness[i] > best_fitness {
                best = i;
                best_fitness = self.fitness[i];
            }
        }
        let avg_len = node_count / popsize;
        avg_fitness /= popsize as f32;

        writeln!(
            self.writer.borrow_mut(),
            "Generation={}
Avg Fitness={}
Best Fitness={}
Avg Size={}",
            self.generation,
            -avg_fitness,
            -best_fitness,
            avg_len
        )
        .unwrap();
        writeln!(self.writer.borrow_mut(), "Best Individual: ").unwrap();
        // writeln!(self.writer.borrow_mut(), "{:?}", self.population[best]);
        // pprint(&self.population[best]);
        writeln!(self.writer.borrow_mut(), "{:?}\n", &self.population[best]).unwrap();

        (best_fitness, best)
    }
}

pub fn random_population(
    params: &Params,
    cases: &Vec<Case>,
    rand: &mut StdRng,
    fitness_func: FitnessFunc
) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(params.popsize);
    let mut fitness = Vec::with_capacity(params.popsize);

    for i in 0..params.popsize {
        population.push(create_random_indiv(params, rand));
        fitness.push(run_and_rank(&population[i], params, cases, fitness_func));
    }

    return (population, fitness);
}

#[cfg(test)]
mod tests {}
