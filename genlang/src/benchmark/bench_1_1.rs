use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::fitness_funcs::*;
use crate::Args;
use super::util::execute_benchmark;

// 1.1.A Program powinien wygenerować na wyjściu (na dowolnej pozycji w danych wyjściowych) liczbę 1. Poza liczbą 1 może też zwrócić inne liczby.
pub fn bench_1_1_a(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 100,
        max_size: 4,
        p_crossover: 0.9,
        p_mut_per_node: 0.2,
        tournament_size: 2,
        random_initial_memory: true,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![1]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1]),
    ];

    execute_benchmark(args, params, cases, "1_1_a", diff_first);
}

// 1.1.B Program powinien wygenerować na wyjściu (na dowolnej pozycji w danych wyjściowych) liczbę 789. Poza liczbą 789 może też zwrócić inne liczby.
pub fn bench_1_1_b(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 100,
        max_size: 4, // ignored during initial generation, low number prevents bloating
        p_crossover: 0.9,
        p_mut_per_node: 0.1,
        tournament_size: 2,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.0,
            max_const: 1000,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![789]),
        (vec![1], vec![789]),
        (vec![1, 2], vec![789]),
    ];

    execute_benchmark(args, params, cases, "1_1_b", diff_first);
}
