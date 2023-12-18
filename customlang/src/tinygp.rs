use crate::params::Case;
use crate::params::Params;

use rand::prelude::*;
use rand::SeedableRng;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::io::Write;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum Funcs {
    Start = 110, // number important for serialization, TODO after the course calculate the index dynamically based on number of variables and const numbers
    ADD,
    SUB,
    MUL,
    DIV,
    SIN,
    COS,
    OUTPUT,
    End, // need to generate ranges, TODO after the course get rid of it along with Funcs::Start
}

const CONST_NUM: usize = 0;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Func(Funcs),
    Val(usize),
}

// const FSET_START: usize = ADD;
// const Funcs::End as usize: usize = DIV + 1;

const MAX_LEN: usize = 10000;

pub type Program = Vec<Opcode>;

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
        let variables: Vec<f32> = (0..Funcs::Start as usize + 1)
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
            fs::write(
                "solution.txt",
                self.equation_string(&self.population[best_id]),
            )
            .unwrap();
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
                child_program = self.crossover(father_id, mother_id);
            } else {
                let parent = tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
                child_program = self.mutation(parent);
            };
            let child_index =
                negative_tournament(&self.fitness, self.params.tournament_size, &mut self.rand);
            self.fitness[child_index] = fitness_func(&child_program, &self.params, &self.cases, &self.variables);
            self.population[child_index] = child_program;
        }
        self.generation += 1;
    }

    fn crossover(&mut self, father_id: usize, mother_id: usize) -> Program {
        let father = &self.population[father_id];
        let mother = &self.population[mother_id];

        let len1 = father.len();
        let len2 = mother.len();

        let xo1start = self.rand.gen_range(0, len1);
        let xo1end = get_expression_end(father, xo1start);

        let xo2start = self.rand.gen_range(0, len2);
        let xo2end = get_expression_end(mother, xo2start);

        let mut offspring: Program =
            Vec::with_capacity(xo1start + (xo2end - xo2start) + (len1 - xo1end));
        offspring.extend_from_slice(&father[0..xo1start]);
        offspring.extend_from_slice(&mother[xo2start..xo2end]);
        offspring.extend_from_slice(&father[xo1end..len1]);
        offspring
    }

    fn mutation(&mut self, parent_id: usize) -> Program {
        let parent = &self.population[parent_id];
        let mut child = Vec::with_capacity(parent.len());
        for i in 0..parent.len() {
            let replacement: Opcode;
            if self.rand.gen_bool(self.params.pmut_per_node as f64) {
                match parent[i] {
                    Opcode::Func(_) => {
                        let nonterminal = self
                            .rand
                            .gen_range(Funcs::Start as usize + 1, Funcs::End as usize);
                        replacement = Opcode::Func(Funcs::from_usize(nonterminal).unwrap());
                    }
                    Opcode::Val(_) => {

                        let terminal = self
                            .rand
                            .gen_range(0, Funcs::Start as usize);
                        replacement = Opcode::Val(terminal);
                    }
                }
            } else {
                replacement = parent[i];
            }
            child.push(replacement);
        }
        child
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
        writeln!(
            self.writer.borrow_mut(),
            "{}\n",
            self.equation_string(&self.population[best])
        )
        .unwrap();

        (best_fitness, best)
    }

    pub fn equation_string(&self, program: &Program) -> String {
        let mut buffer = String::with_capacity(program.len() * 3);
        self.serialize_equation_string(program, &mut 0, &mut buffer);
        buffer
    }

    fn serialize_equation_string(
        &self,
        program: &Program,
        cursor: &mut usize,
        buffer: &mut String,
    ) {
        let opcode = program[*cursor];

        let mut infix = |sign: &str| {
            *buffer += "(";
            *cursor += 1;
            self.serialize_equation_string(program, cursor, buffer);
            *buffer += sign;
            *cursor += 1;
            self.serialize_equation_string(program, cursor, buffer);
            *buffer += ")";
        };

        match opcode {
            Opcode::Func(funcid) => match funcid {
                Funcs::ADD => {
                    infix(" + ");
                }
                Funcs::SUB => {
                    infix(" - ");
                }
                Funcs::MUL => {
                    infix(" * ");
                }
                Funcs::DIV => {
                    infix(" / ");
                }
                Funcs::SIN => {
                    *buffer += "sin(";
                    *cursor += 1;
                    self.serialize_equation_string(program, cursor, buffer);
                    *buffer += ")";
                }
                Funcs::COS => {
                    *buffer += "cos(";
                    *cursor += 1;
                    self.serialize_equation_string(program, cursor, buffer);
                    *buffer += ")";
                }
                Funcs::OUTPUT => {
                    *buffer += "OUTPUT";
                }
                Funcs::Start => unreachable!("Funcs::Start"),
                Funcs::End => unreachable!("Funcs::End"),
            },
            Opcode::Val(x) => {

                if x < Funcs::Start as usize - CONST_NUM {
                    *buffer += format!("X{}", x + 1).as_str();
                } else {
                    *buffer += format!("{}", self.variables[x]).as_str();
                }
            }
        };
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
fn grow_stat(program: &mut Program, depth: usize, params: &Params, rand: &mut StdRng) -> bool {
    if program.len() >= MAX_LEN {
        return false;
    }

    if depth > 0 && rand.gen_bool(0.5) {
        // generate operation
        let operation = rand.gen_range(Funcs::Start as usize + 1, Funcs::End as usize);
        program.push(Opcode::Func(Funcs::from_usize(operation).unwrap()));
        // generate operands
        if !grow_stat(program, depth - 1, params, rand) {
            return false;
        }
        return grow_stat(program, depth - 1, params, rand);
    } else {
        let terminal: usize = rand.gen_range(0, params.memsize) as usize;
        program.push(Opcode::Val(terminal));
        return true;
    }
}

fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(2 * params.depth);
    grow_stat(&mut program, params.depth, params, rand);
    program
}

fn fitness_func(program: &Program, params: &Params, cases: &Vec<Case>, variables: &Vec<f32>) -> f32 {
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
    println!("{:?}", program);
    let mut memory = vec![0.0; params.memsize];
    let mut cursor = 0;
    let mut output: Vec<f32> = Vec::new();
    eval_stat(program, &mut memory, &mut cursor, &mut output);
    return *output.get(0).unwrap_or(&1.0);
}

fn read_reg(token: Opcode, memory: &Vec<f32>) -> f32 {
    match token {
        Opcode::Val(num) => {
            memory.get(num).unwrap().clone()
        },
        _ => {
            unreachable!()
        }
    }
}

fn eval_stat(program: &Program, memory: &mut Vec<f32>, cursor: &mut usize, output: &mut Vec<f32>) {
    match program[*cursor] {
        Opcode::Func(keyword) => match keyword {
            Funcs::OUTPUT => {
                let regval = read_reg(program[*cursor + 1], &memory);
                output.push(regval);
            },
            _ => unreachable!()
        },
        Opcode::Val(_) => unreachable!()
    }
}

fn eval_expr(program: &Program, memory: &Vec<f32>, cursor: &mut usize) -> f32 {
    let opcode = program[*cursor];
    *cursor += 1;

    return match opcode {
        Opcode::Func(func) => match func {
            Funcs::ADD => eval_expr(program, memory, cursor) + eval_expr(program, memory, cursor),
            Funcs::SUB => eval_expr(program, memory, cursor) - eval_expr(program, memory, cursor),
            Funcs::MUL => eval_expr(program, memory, cursor) * eval_expr(program, memory, cursor),
            Funcs::DIV => {
                let numerator = eval_expr(program, memory, cursor);
                let denominator = eval_expr(program, memory, cursor);
                if denominator.abs() <= 0.001 {
                    numerator
                } else {
                    numerator / denominator
                }
            }
            Funcs::SIN => f32::sin(eval_expr(program, memory, cursor)),
            Funcs::COS => f32::cos(eval_expr(program, memory, cursor)),
            _ => unreachable!()
        },
        Opcode::Val(i) => memory[i],
    };
}

fn get_expression_end(program: &Program, start: usize) -> usize {
    match program[start] {
        Opcode::Val(_) => start + 1,
        Opcode::Func(_) => get_expression_end(program, get_expression_end(program, start + 1)),
    }
}

#[allow(unused)]
fn pprint_recurse(program: &Program, cursor: &mut usize, buffer: &mut String, indent: usize) {
    if *cursor >= program.len() {
        return;
    }

    let opcode = program[*cursor];
    *cursor += 1;
    match opcode {
        Opcode::Val(i) => {
            *buffer += format!("{}{}\n", str::repeat(" ", indent), i).as_str();
        }
        Opcode::Func(func) => {
            *buffer += format!(
                "{}{}\n",
                str::repeat(" ", indent),
                match func {
                    Funcs::ADD => "ADD",
                    Funcs::SUB => "SUB",
                    Funcs::MUL => "MUL",
                    Funcs::DIV => "DIV",
                    Funcs::SIN => "SIN",
                    Funcs::COS => "COS",
                    Funcs::OUTPUT => "OUTPUT",
                    Funcs::Start => unreachable!(),
                    Funcs::End => unreachable!(),
                }
            )
            .as_str();

            pprint_recurse(program, cursor, buffer, indent + 2);
            pprint_recurse(program, cursor, buffer, indent + 2);
        }
    }
}

#[allow(unused)]
fn pprint(program: &Program, writer: RefCell<Box<dyn Write>>) {
    let mut s = String::new();
    let mut cursor = 0;
    pprint_recurse(program, &mut cursor, &mut s, 0);
    writeln!(writer.borrow_mut(), "{}", s);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io;

    #[test]
    fn test_execute() {
        let program: Vec<Opcode> = vec![
            Opcode::Func(Funcs::ADD),
            Opcode::Val(0),
            Opcode::Func(Funcs::DIV),
            Opcode::Val(1),
            Opcode::Val(1),
        ];
        let data = vec![1.0, -2.0];
        assert_eq!(2.0, eval_expr(&program, &data, &mut 0));

        let program: Vec<Opcode> = vec![
            Opcode::Func(Funcs::SUB),
            Opcode::Val(0),
            Opcode::Func(Funcs::DIV),
            Opcode::Val(1),
            Opcode::Val(2),
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

    // #[test]
    // fn test_fitness() {
    //     let program = vec![
    //         Opcode::Func(Funcs::ADD),
    //         Opcode::Val(0),
    //         Opcode::Func(Funcs::DIV),
    //         Opcode::Val(1),
    //         Opcode::Val(1),
    //     ];

    //     let cases: Vec<Case> = vec![(vec![1.0], vec![2.0])];
    //     let mut variables: Vec<f32> = vec![0.0; 1];
    //     variables.push(2.0);
    //     let result = fitness_func(&program, &cases, &variables);
    //     assert_eq!(result, 0.0);

    //     let cases: Vec<Case> = vec![(vec![1.0], vec![0.0]), (vec![1.0, 2.0], vec![0.0])];
    //     let result = fitness_func(&program, &cases, &variables);
    //     assert_eq!(result, -4.0);
    // }

    fn mock_params() -> Params {
        Params {
            seed: 1,
            popsize: 10,
            memsize: 10,
            depth: 3,
            crossover_prob: 0.9,
            pmut_per_node: 0.1,
            tournament_size: 2,
            acceptable_error: -1e-5,
        }
    }

    #[test]
    fn test_grow_depth_0() {
        let mut program = Vec::new();
        let mut rand: StdRng = StdRng::seed_from_u64(1);
        grow_stat(&mut program, 0, &mock_params(), &mut rand);
        assert!(program.len() == 1)
    }

    #[test]
    fn test_print_indiv() {
        let t = TinyGP::new(
            mock_params(),
            Vec::new(),
            Some(5),
            RefCell::new(Box::new(io::stdout())),
        );
        let s = t.equation_string(&vec![
            Opcode::Func(Funcs::ADD),
            Opcode::Val(0),
            Opcode::Val(0),
        ]);
        assert_eq!(s, "(X1 + X1)")
    }

    #[test]
    fn test_get_expression_end() {
        let program = vec![Opcode::Func(Funcs::ADD), Opcode::Val(0), Opcode::Val(0)];
        assert_eq!(get_expression_end(&program, 0), 3);
        assert_eq!(get_expression_end(&program, 1), 2);
        assert_eq!(get_expression_end(&program, 2), 3);
        let program = vec![
            Opcode::Func(Funcs::ADD),
            Opcode::Func(Funcs::ADD),
            Opcode::Val(0),
            Opcode::Val(0),
            Opcode::Val(0),
        ];
        assert_eq!(get_expression_end(&program, 0), 5);
        assert_eq!(get_expression_end(&program, 1), 4);
        assert_eq!(get_expression_end(&program, 2), 3);
        assert_eq!(get_expression_end(&program, 3), 4);
        assert_eq!(get_expression_end(&program, 4), 5);
    }

    #[test]
    fn test_pprint() {
        let program = vec![
            Opcode::Func(Funcs::ADD),
            Opcode::Func(Funcs::ADD),
            Opcode::Val(0),
            Opcode::Val(0),
            Opcode::Val(0),
        ];
        let mut s = String::new();
        let mut cursor = 0;
        pprint_recurse(&program, &mut cursor, &mut s, 0);
        assert_eq!(
            "ADD
  ADD
    0
    0
  0
",
            s
        );

    }

    #[test]
    fn test_sin() {
        let program = vec![
            Opcode::Func(Funcs::SIN),
            Opcode::Val(0),
        ];

        assert_eq!(
            0.0,
            eval_expr(
                &program,
                &vec![0.0],
                &mut 0
            )
        );
    }

    #[test]
    fn test_cos() {
        let program = vec![
            Opcode::Func(Funcs::COS),
            Opcode::Val(0),
        ];

        assert_eq!(
            1.0,
            eval_expr(
                &program,
                &vec![0.0],
                &mut 0
            )
        );
    }
}
