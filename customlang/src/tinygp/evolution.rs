use crate::params::Params;

use super::common::*;
use rand::prelude::*;

pub fn crossover(father: &Program, mother: &Program, rand: &mut StdRng) -> Program {
    log::trace!("crossover {father:?} x {mother:?}");

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

pub fn mutation(parent: &Program, params: &Params, rand: &mut StdRng) -> Program {
    log::trace!("mutation");
    let mut child = Vec::with_capacity(parent.len());
    for i in 0..parent.len() {
        let replacement: Token;
        if rand.gen_bool(params.pmut_per_node as f64) {
            match parent[i] {
                Token::Expr(e) => {
                    let nonterminal: Expr = rand.gen();
                    if e.argnum() == nonterminal.argnum() {
                        replacement = Token::Expr(nonterminal);
                    } else {
                        log::warn!("mutation for different argument numbers skipped");
                        replacement = Token::Expr(e);
                    }
                }
                Token::Reg(_) => {
                    replacement = Token::Reg(rand.gen_range(0, params.memsize));
                }
                Token::Stat(stat) => {
                    replacement = Token::Stat(stat);
                },
            }
        } else {
            replacement = parent[i];
        }
        child.push(replacement);
    }
    child
}

pub fn tournament(fitness: &Vec<f32>, tournament_size: usize, rand: &mut StdRng) -> usize {
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

pub fn negative_tournament(fitness: &Vec<f32>, tournament_size: usize, rand: &mut StdRng) -> usize {
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
