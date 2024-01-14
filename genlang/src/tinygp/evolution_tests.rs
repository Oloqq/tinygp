use std::io::{self, Write};

use crate::params::{Params, Case, GrowingParams};

use super::{
    // common::*,
    // evolution::*,
    TinyGP
};

#[test]
fn test_e2e_identity() {
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 100,
        max_depth: 3,
        max_size: 4,
        p_crossover: 0.9,
        p_mut_per_node: 0.05,
        tournament_size: 2,
        acceptable_error: 0.1,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1], vec![1]),
        (vec![4], vec![4]),
        (vec![10], vec![10])
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into());
    let (program, fitness) = tgp.evolve(3);
    println!("{:?}", tgp.population);
    println!("{:?}", tgp.fitness);
    println!("{:?}", program);
    assert_eq!(fitness, 0.0);
}

#[test]
fn test_e2e_gen_1() { // 1.1.A
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 100,
        max_depth: 3,
        max_size: 4,
        p_crossover: 0.9,
        p_mut_per_node: 0.05,
        tournament_size: 2,
        acceptable_error: 0.1,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1])
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into());
    let (program, fitness) = tgp.evolve(3);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}

#[test]
fn test_e2e_gen_789() { // 1.1.B
    let params = Params {
        seed: 0,
        memsize: 3,
        popsize: 100,
        max_depth: 3,
        max_size: 4,
        acceptable_error: 0.0,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.0,
            min_const: 789,
            max_const: 790,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![], vec![789]),
        (vec![], vec![789]),
        (vec![], vec![789])
    ];
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let seed = Some(0);
    let mut tgp = TinyGP::new(params, cases, seed, writer.into());
    let (program, fitness) = tgp.evolve(3);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}