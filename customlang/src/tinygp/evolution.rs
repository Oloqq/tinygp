use crate::params::Params;

use super::common::*;
use rand::prelude::*;
use num_traits::FromPrimitive;

pub fn crossover(father: &Program, mother: &Program, rand: &mut StdRng) -> Program {
    println!("crossover {:?} x {:?}", father, mother);

    let len1 = father.len();
    let len2 = mother.len();

    let xo1start = rand.gen_range(0, len1);
    let xo1end = get_node_end(father, xo1start);

    let xo2start = rand.gen_range(0, len2);
    let xo2end = get_node_end(mother, xo2start);
    println!("{xo1start}, {xo1end}, {xo2start}, {xo2end}");

    let mut offspring: Program =
        Vec::with_capacity(xo1start + (xo2end - xo2start) + (len1 - xo1end));
    offspring.extend_from_slice(&father[0..xo1start]);
    offspring.extend_from_slice(&mother[xo2start..xo2end]);
    offspring.extend_from_slice(&father[xo1end..len1]);
    println!(" -> {:?}", offspring);
    offspring
}

pub fn mutation(parent: &Program, params: &Params, rand: &mut StdRng) -> Program {
    println!("mutation");
    let mut child = Vec::with_capacity(parent.len());
    for i in 0..parent.len() {
        let replacement: Token;
        if rand.gen_bool(params.pmut_per_node as f64) {
            match parent[i] {
                Token::Kw(_) => {
                    let nonterminal = rand.gen_range(Funcs::Start as usize + 1, Funcs::End as usize);
                    replacement = Token::Kw(Funcs::from_usize(nonterminal).unwrap());
                }
                Token::Reg(_) => {
                    let terminal = rand.gen_range(0, Funcs::Start as usize);
                    replacement = Token::Reg(terminal);
                }
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