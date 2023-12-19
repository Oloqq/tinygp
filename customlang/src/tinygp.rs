mod common;
mod evolution;
mod growing;

use crate::params::Case;
use crate::params::Params;
use common::*;
use evolution::*;
use growing::*;

use rand::prelude::*;
use rand::SeedableRng;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::io::Write;

struct Context {
    memory: Vec<f32>,
    input: Vec<f32>,
    output: Vec<f32>,
    input_cursor: usize,
}

impl Context {
    pub fn new(memsize: usize, input: Vec<f32>) -> Self {
        Context {
            memory: vec![0.0; memsize],
            input,
            output: Vec::new(),
            input_cursor: 0,
        }
    }

    pub fn next_input(&mut self) -> Option<f32> {
        if self.input.len() < self.input_cursor {
            let val = self.input[self.input_cursor];
            self.input_cursor += 1;
            Some(val)
        } else {
            None
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
enum EvalError {
    Finished,
    Syntax,
    Semantic,
}

pub struct TinyGP {
    rand: StdRng,
    params: Params,
    cases: Vec<Case>,
    generation: i32,
    population: Vec<Program>,
    fitness: Vec<f32>,
    variables: Vec<f32>,
    writer: RefCell<Box<dyn Write>>,
}

impl TinyGP {
    fn new(
        mut params: Params,
        cases: Vec<Case>,
        seed: Option<u64>,
        writer: RefCell<Box<dyn Write>>,
    ) -> TinyGP {
        let seed = seed.unwrap_or(StdRng::from_entropy().next_u64());
        let mut rand = StdRng::seed_from_u64(seed);
        params.seed = seed;
        const MIN_RANDOM: f32 = -20.0;
        const MAX_RANDOM: f32 = 20.0;
        writeln!(writer.borrow_mut(), "Creating variables").unwrap();
        let variables: Vec<f32> = (0..Expr::ADD as usize + 1)
            .map(|_| rand.gen_range(MIN_RANDOM, MAX_RANDOM))
            .collect();
        writeln!(writer.borrow_mut(), "Creating population").unwrap();
        let (population, fitness) = random_population(&params, &cases, &mut rand, &variables);
        TinyGP {
            rand,
            fitness,
            population,
            params,
            cases,
            generation: 0,
            variables,
            writer: writer.into(),
        }
    }

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
        Ok(TinyGP::new(params, cases, seed, writer))
    }

    pub fn evolve(&mut self, generations: usize) {
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
            self.evolve_generation();
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
    }

    fn evolve_generation(&mut self) {
        for _ in 0..self.params.popsize {
            let child_program: Program;
            if self.rand.gen_bool(self.params.crossover_prob as f64) {
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
            self.fitness[child_index] =
                fitness_func(&child_program, &self.params, &self.cases, &self.variables);
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

fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(2 * params.depth);
    grow_stat(&mut program, params.depth, params, rand);
    program
}

fn fitness_func(
    program: &Program,
    params: &Params,
    cases: &Vec<Case>,
    variables: &Vec<f32>,
) -> f32 {
    let mut vars = variables.clone();
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        vars.splice(0..inputs.len(), inputs.iter().cloned());
        let output = execute(program, params);
        let error = (output - targets[0]).abs();
        acc - error
    })
}

fn random_population(
    params: &Params,
    cases: &Vec<Case>,
    rand: &mut StdRng,
    variables: &Vec<f32>,
) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(params.popsize);
    let mut fitness = Vec::with_capacity(params.popsize);

    for i in 0..params.popsize {
        population.push(create_random_indiv(params, rand));
        fitness.push(fitness_func(&population[i], params, cases, variables));
    }

    return (population, fitness);
}

fn execute(program: &Program, params: &Params) -> f32 {
    log::trace!("executing {:?}", program);
    let mut ctx = Context::new(params.memsize, vec![]);
    let mut cursor = 0;

    match eval_stat(program, &mut cursor, &mut ctx) {
        Ok(_) => {}
        Err(e) => match e {
            EvalError::Finished => {}
            EvalError::Syntax => todo!(),
            EvalError::Semantic => todo!(),
        },
    }
    return *ctx.output.get(0).unwrap_or(&1.0);
}

fn read_reg(token: Token, memory: &Vec<f32>) -> f32 {
    match token {
        Token::Reg(num) => memory.get(num).unwrap().clone(),
        _ => {
            unreachable!()
        }
    }
}

fn eval_stat(program: &Program, cursor: &mut usize, ctx: &mut Context) -> Result<(), EvalError> {
    if let Token::Stat(stat) = program[*cursor] {
        match stat {
            Stat::OUTPUT => {
                let regval = read_reg(program[*cursor + 1], &ctx.memory);
                ctx.output.push(regval);
            }
            Stat::INPUT => {
                let regnum = match program[*cursor + 1] {
                    Token::Reg(num) => num,
                    _ => panic!(
                        "Expected Reg at {}, got {:?}",
                        *cursor + 1,
                        program[*cursor + 1]
                    ),
                };
                let val = match ctx.next_input() {
                    Some(val) => val,
                    None => return Err(EvalError::Finished),
                };
                ctx.memory[regnum] = val;
            }
        }
        return Ok(());
    }
    panic!("called eval_stat on non-stat");
}

#[allow(unused)]
fn eval_expr(program: &Program, memory: &Vec<f32>, cursor: &mut usize) -> f32 {
    let opcode = program[*cursor];
    *cursor += 1;

    return match opcode {
        Token::Expr(func) => match func {
            Expr::ADD => eval_expr(program, memory, cursor) + eval_expr(program, memory, cursor),
            Expr::SUB => eval_expr(program, memory, cursor) - eval_expr(program, memory, cursor),
            Expr::MUL => eval_expr(program, memory, cursor) * eval_expr(program, memory, cursor),
            Expr::DIV => {
                let numerator = eval_expr(program, memory, cursor);
                let denominator = eval_expr(program, memory, cursor);
                if denominator.abs() <= 0.001 {
                    numerator
                } else {
                    numerator / denominator
                }
            }
            Expr::SIN => f32::sin(eval_expr(program, memory, cursor)),
            Expr::COS => f32::cos(eval_expr(program, memory, cursor)),
            _ => unreachable!(),
        },
        Token::Reg(i) => memory[i],
        Token::Stat(_) => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let program: Vec<Token> = vec![
            Token::Expr(Expr::ADD),
            Token::Reg(0),
            Token::Expr(Expr::DIV),
            Token::Reg(1),
            Token::Reg(1),
        ];
        let data = vec![1.0, -2.0];
        assert_eq!(2.0, eval_expr(&program, &data, &mut 0));

        let program: Vec<Token> = vec![
            Token::Expr(Expr::SUB),
            Token::Reg(0),
            Token::Expr(Expr::DIV),
            Token::Reg(1),
            Token::Reg(2),
        ];
        assert_eq!(
            0.8776571,
            eval_expr(
                &program,
                &vec![0.0, -4.025456902691228, 4.58659426408455],
                &mut 0
            )
        );
    }

    #[test]
    fn test_replace() {
        let mut vars = vec![1; 2];
        vars.push(1);
        let inputs = vec![9];
        vars.splice(0..inputs.len(), inputs.iter().cloned());
        assert_eq!(vars.len(), 3);
        assert_eq!(vars[0], 9);
        assert_eq!(vars[1], 1);
        assert_eq!(vars[2], 1);
    }

    #[test]
    fn test_sin() {
        let program = vec![Token::Expr(Expr::SIN), Token::Reg(0)];

        assert_eq!(0.0, eval_expr(&program, &vec![0.0], &mut 0));
    }

    #[test]
    fn test_cos() {
        let program = vec![Token::Expr(Expr::COS), Token::Reg(0)];

        assert_eq!(1.0, eval_expr(&program, &vec![0.0], &mut 0));
    }
}
