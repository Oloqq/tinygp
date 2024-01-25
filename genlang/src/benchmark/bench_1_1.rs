use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::fitness_funcs::*;
use crate::tinygp::TinyGP;

use std::cell::RefCell;
use std::fs::File;
use std::io::{self, Write};

pub fn bench_1_1_a(seed: Option<u64>, fresh: bool, generations: usize) {
    const POP_FILE: &str = "population/1_1_a";
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
        (vec![1, 2], vec![1]),
    ];

    let writer: RefCell<Box<dyn Write>> = RefCell::new(Box::new(io::stdout()));
    let mut tgp;
    if !fresh {
        tgp = match TinyGP::from_population(&params, &cases, seed, writer, diff_first, POP_FILE) {
            Ok(tgp) => tgp,
            Err(_) => {
                println!("Couldn't load previous population, starting fresh");
                let writer: RefCell<Box<dyn Write>> = RefCell::new(Box::new(io::stdout()));
                TinyGP::new(params, cases, seed, writer, diff_first)
            }
        }
    } else {
        tgp = TinyGP::new(params, cases, seed, writer, diff_first);
    }

    let (program, fitness) = tgp.evolve(generations, diff_first);
    let mut writer: Box<dyn Write> =
        Box::new(File::create(POP_FILE).expect("Could not create file"));

    tgp.save_population(&mut writer);
    println!("{:?}", program);
    println!("{:?}", fitness);
    assert_eq!(fitness, 0.0);
}
