// use std::io::{self, Write};

// use crate::params::{Params, Case};

// use super::{
//     // common::*,
//     // evolution::*,
//     TinyGP
// };

// #[test]
// #[ignore]
// fn test_e2e_2_number_sum() {
//     // let params = Params {
//     //     seed: 0,
//     //     memsize: 3,
//     //     popsize: 10,
//     //     max_depth: 3,
//     //     crossover_prob: 0.9,
//     //     pmut_per_node: 0.05,
//     //     tournament_size: 2,
//     //     acceptable_error: 0.1,
//     // };
//     // let cases: Vec<Case> = vec![
//     //     (vec![1, 2], vec![3]),
//     //     (vec![-1, 10], vec![9])
//     // ];
//     // let writer: Box<dyn Write> = Box::new(io::stdout());
//     // let seed = Some(0);
//     // let mut tgp = TinyGP::new(params, cases, seed, writer.into());
//     // tgp.evolve(3);
//     // println!("{:?}", tgp.population);
//     // println!("{:?}", tgp.fitness);
//     // assert!(false);
// }