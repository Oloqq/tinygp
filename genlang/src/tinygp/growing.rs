use crate::params::{Params, Case};
use super::common::*;
use super::execution::*;
use rand::prelude::*;

pub fn grow_stat(program: &mut Program, depth: usize, params: &Params, rand: &mut StdRng) -> bool {
    if program.len() >= MAX_LEN || depth > params.depth {
        return false;
    }
    let stat = rand.gen();
    program.push(Token::Stat(stat));
    match stat {
        Stat::OUTPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            program.push(reg);
        }
        Stat::INPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            program.push(reg);
        },
        _ => todo!()
    }
    log::trace!("grew into {:?}", program);
    return true;
}

pub fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(2 * params.depth);
    program.append(&mut vec![Token::Stat(Stat::INPUT), Token::Reg(0)]);
    grow_stat(&mut program, params.depth, params, rand);
    grow_stat(&mut program, params.depth, params, rand);
    program.append(&mut vec![Token::Stat(Stat::OUTPUT), Token::Reg(0)]);
    program
}

pub fn fitness_func(program: &Program, params: &Params, cases: &Vec<Case>) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let runtime = Runtime::new(params.memsize, inputs.clone()); // TODO dont clone inputs, not needed
        let output = execute(program, runtime);
        let output = output.get(0).unwrap_or(&0); // FIXME
        let error = (output - targets[0]).abs();
        let fitness = acc - error as f32;
        log::trace!("the fitness is: {fitness}");
        fitness
    })
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
            depth: 3,
            ..Default::default()
        };
        let mut rand = StdRng::seed_from_u64(0);
        let _prog = create_random_indiv(&params, &mut rand);
    }
}
