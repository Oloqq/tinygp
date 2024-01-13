use crate::params::{Params, Case};
use super::common::*;
use super::execution::*;
use rand::prelude::*;

pub fn fitness_func(program: &Program, params: &Params, cases: &Vec<Case>) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let runtime = Runtime::new(params.memsize, inputs.clone()); // TODO dont clone inputs, not needed
        let output = execute(program, runtime);
        let output = output.get(0).unwrap_or(&0);
        let error = (output - targets[0]).abs();
        let fitness = acc - error as f32;
        log::trace!("the fitness is: {fitness}");
        fitness
    })
}

pub fn grow_stat(size_left: i32, _depth_left: usize, params: &Params, rand: &mut StdRng) -> Vec<Token> {
    let stat: Stat = rand.gen();
    let mut code: Vec<Token> = vec![];
    code.push(Token::Stat(stat));
    match stat {
        Stat::OUTPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            code.push(reg);
        }
        Stat::INPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            code.push(reg);
        },
        _ => {
            log::error!("growing logic unfinished");
            return vec![];
        }
    }
    return if size_left > code.len() as i32 { code } else { vec![] };
}

pub fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(2 * params.max_depth);
    program.append(&mut vec![Token::Stat(Stat::INPUT), Token::Reg(0)]);
    program.append(&mut grow_stat(i32::MAX, params.max_depth, params, rand));
    program.append(&mut grow_stat(i32::MAX, params.max_depth, params, rand));
    program.append(&mut vec![Token::Stat(Stat::OUTPUT), Token::Reg(0)]);
    program
}

pub fn random_population(
    params: &Params,
    cases: &Vec<Case>,
    rand: &mut StdRng,
) -> (Vec<Program>, Vec<f32>) {
    let mut population = Vec::with_capacity(params.popsize);
    let mut fitness = Vec::with_capacity(params.popsize);

    for i in 0..params.popsize {
        population.push(create_random_indiv(params, rand));
        fitness.push(fitness_func(&population[i], params, cases));
    }

    return (population, fitness);
}

#[cfg(test)]
mod tests {
    use super::*;

    // const ANY_PARAMS: Params = Params {
    //     seed: 0,
    //     memsize: 3,
    //     popsize: 10,
    //     depth: 3,
    //     crossover_prob: 0.9,
    //     pmut_per_node: 0.05,
    //     tournament_size: 2,
    //     acceptable_error: 0.1,
    // };

    #[test]
    #[ignore]
    fn test_random_indiv() {
        let params = Params {
            max_depth: 3,
            ..Default::default()
        };
        let mut rand = StdRng::seed_from_u64(0);
        let prog = create_random_indiv(&params, &mut rand);
        assert!(prog.len() > 4);
        assert!(matches!(prog[0], Token::Stat(Stat::INPUT)));
        assert!(matches!(prog[prog.len() - 2], Token::Stat(Stat::OUTPUT)));
    }
}
