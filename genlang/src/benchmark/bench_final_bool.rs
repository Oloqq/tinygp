use super::util::execute_benchmark;
use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::common::{Expr, Stat};
use crate::tinygp::fitness_funcs::*;
use crate::Args;

// Regresja symboliczna dla funkcji boolowskiej - k = [1,10]

pub fn bench_final_bool_1(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![

    ];

    execute_benchmark(args, params, cases, "final_bool_1", fit_bool);
}

pub fn bench_final_bool_2(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_2", fit_bool);
}

pub fn bench_final_bool_3(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_3", fit_bool);
}

pub fn bench_final_bool_4(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_4", fit_bool);
}

pub fn bench_final_bool_5(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_5", fit_bool);
}

pub fn bench_final_bool_6(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_6", fit_bool);
}

pub fn bench_final_bool_7(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_7", fit_bool);
}

pub fn bench_final_bool_8(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_8", fit_bool);
}

pub fn bench_final_bool_9(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_9", fit_bool);
}

pub fn bench_final_bool_10(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![];

    execute_benchmark(args, params, cases, "final_bool_10", fit_bool);
}
