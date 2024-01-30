use super::execution::*;
use super::fitness_funcs::*;
use super::growing::rand_const;
use crate::params::{Case, Params};

use super::common::*;
use super::growing::rand_reg;
use rand::prelude::*;

pub fn run_and_rank(
    program: &Program,
    params: &Params,
    cases: &Vec<Case>,
    fitness_func: FitnessFunc,
) -> f32 {
    cases.iter().fold(0.0, |acc, (inputs, targets)| {
        let runtime = Runtime::new(params.memsize, &inputs);
        let output = execute(program, runtime);
        let fitness = fitness_func(targets, &output);
        // log::trace!("the fitness is: {fitness}");
        acc + fitness
    })
}

pub fn crossover(father: &Program, mother: &Program, rand: &mut StdRng) -> Program {
    log::debug!("crossover {father:?} x {mother:?}");

    let father_start = rand.gen_range(0, father.len());
    let father_kind = father[father_start];
    let father_end = get_node_end(father, father_start);

    let mother_start = match mother
        .iter()
        .enumerate()
        .filter(|(_i, v)| variant_eq(&father_kind, &v))
        .choose(rand)
    {
        Some((i, _v)) => i,
        None => {
            log::warn!("parents non compatible, returning father");
            return father.clone();
        }
    };
    let mother_end = get_node_end(mother, mother_start);

    let mut offspring: Program = Vec::with_capacity(
        father_start + (mother_end - mother_start) + (father.len() - father_end),
    );
    offspring.extend_from_slice(&father[0..father_start]);
    offspring.extend_from_slice(&mother[mother_start..mother_end]);
    offspring.extend_from_slice(&father[father_end..father.len()]);
    log::trace!(" -> {offspring:?}");
    offspring
}

fn mutate_expression(source: Expr, params: &Params, rand: &mut StdRng) -> Token {
    let replacement: Token;
    // TODO implement reductive mutation (allow mutating with different argnum) (truncate the rest of the tree)
    let candidate: Expr = {
        let mut cand: Expr = rand.gen();
        while source.argnum() != cand.argnum() {
            cand = rand.gen();
        }
        cand
    };

    if matches!(candidate, Expr::Reg(_)) {
        replacement = rand_reg(params, rand);
    } else if matches!(candidate, Expr::Num(_)) {
        replacement = rand_const(params, rand);
    } else {
        replacement = Token::Expr(candidate);
    }
    replacement
}

pub fn mutation(parent: &Program, params: &Params, rand: &mut StdRng) -> Program {
    log::debug!("mutation");
    let mut child = Vec::with_capacity(parent.len());
    for i in 0..parent.len() {
        let replacement: Token;
        if rand.gen_bool(params.p_mut_per_node as f64) {
            replacement = match parent[i] {
                Token::Expr(e) => mutate_expression(e, params, rand),
                Token::Reg(_) => Token::Reg(rand.gen_range(0, params.memsize)),
                Token::Stat(stat) => Token::Stat(stat),
                _ => unimplemented!(),
            }
        } else {
            replacement = parent[i];
        }
        child.push(replacement);
    }
    child
}

pub fn tournament(fitness: &Vec<f64>, tournament_size: usize, rand: &mut StdRng) -> usize {
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

pub fn negative_tournament(fitness: &Vec<f64>, tournament_size: usize, rand: &mut StdRng) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::params::GrowingParams;

    use super::*;

    #[test]
    fn test_mutate_expression() {
        let params = Params {
            memsize: 5,
            growing: GrowingParams {
                min_const: -1000,
                max_const: 1000,
                ..Default::default()
            },
            ..Default::default()
        };
        let seed = StdRng::from_entropy().next_u64();
        let mut rand = StdRng::seed_from_u64(seed);
        let source = Expr::Num(2);
        let got = mutate_expression(source, &params, &mut rand);
        match got {
            Token::Expr(Expr::Num(x)) => {
                assert!(x >= params.growing.min_const);
                assert!(x < params.growing.max_const);
            }
            #[allow(unused_comparisons)]
            Token::Reg(r) => {
                assert!(r >= 0);
                assert!(r < params.memsize);
            }
            Token::Expr(Expr::Reg(_)) => {
                panic!("mutated into the code smell Expr::R, got: {got:?}, seed = {seed}")
            }
            _ => panic!("mutation went wrong, got: {got:?}, seed = {seed}"),
        }
        assert!(matches!(got, Token::Expr(_)));
    }
}
