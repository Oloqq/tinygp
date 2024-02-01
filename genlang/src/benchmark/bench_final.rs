use super::util::execute_benchmark;
use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::common::{Expr, Stat};
use crate::tinygp::fitness_funcs::*;
use crate::Args;

// Given 3 integers start, end and step, print the integers in the arithemtic sequence
pub fn bench_final_1(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            d_stat: vec![(Stat::IF, 0)],
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![4, 12, 1], vec![4, 5, 6, 7, 8, 9, 10, 11]),
        (vec![0, 100, 20], vec![0, 20, 40, 60, 80]),
        (vec![1, 5, 2], vec![1, 3]),
        (vec![-10, 20, 5], vec![-10, -5, 0, 5, 10, 15]),
        (vec![-15, -4, 3], vec![-15, -12, -9, -6]),
        (vec![10, -10, -3], vec![10, 7, 4, 1, -2, -5, -8]),
        (vec![5, 0, -1], vec![5, 4, 3, 2, 1]),
        (vec![100, 10, -15], vec![100, 85, 70, 55, 40, 25]),
    ];

    execute_benchmark(args, params, cases, "final_1", fit_arithmetic_series);
}

fn sum_of_squares_in_range(to: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..to+1 {
        sum += i*i;
    }
    sum
}

// Given integer n, return the sum of squaring each integer in the range [1, n]
pub fn bench_final_2(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            d_stat: vec![(Stat::IF, 0), (Stat::WHILE, 1)],
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1], vec![sum_of_squares_in_range(1)]),
        (vec![2], vec![sum_of_squares_in_range(2)]),
        (vec![5], vec![sum_of_squares_in_range(5)]),
        (vec![8], vec![sum_of_squares_in_range(8)]),
        (vec![20], vec![sum_of_squares_in_range(20)]),
    ];

    execute_benchmark(args, params, cases, "final_2", diff_first_promote_single);
}

// Print the smallest of 4 integers
pub fn bench_final_3(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        memsize: 15,
        growing: GrowingParams {
            d_stat: vec![(Stat::IF, 0)],
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1, 2, 3, 4], vec![1]),
        (vec![3, 6, 0, 9], vec![0]),
        (vec![-6, 45, 15, 4], vec![-6]),
        (vec![10, 5, 35, 15], vec![5]),
        (vec![10, 10, 10, 2], vec![2]),
    ];

    execute_benchmark(args, params, cases, "final_3", diff_first_promote_single);
}
