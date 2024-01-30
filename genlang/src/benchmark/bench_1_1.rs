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

// 1.1.C Program powinien wygenerować na wyjściu (na dowolnej pozycji w danych wyjściowych) liczbę 31415. Poza liczbą 31415 może też zwrócić inne liczby.
pub fn bench_1_1_c(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 1000,
        max_size: 4,
        p_crossover: 0.5,
        p_mut_per_node: 0.1,
        tournament_size: 2,
        growing: GrowingParams {
            max_const: 35000,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0], vec![31415]),
        (vec![1], vec![31415]),
        (vec![1, 2], vec![31415]),
    ];

    execute_benchmark(args, params, cases, "1_1_c", diff_first);
}

// 1.1.D Program powinien wygenerować na pierwszej pozycji na wyjściu liczbę 1. Poza liczbą 1 może też zwrócić inne liczby.
// rozwiazane przez 1.1.A

// 1.1.E Program powinien wygenerować na pierwszej pozycji na wyjściu liczbę 789. Poza liczbą 789 może też zwrócić inne liczby.
// rozwiazane przez 1.1.B

// 1.1.F Program powinien wygenerować na wyjściu liczbę jako jedyną liczbę 1. Poza liczbą 1 NIE powinien nic więcej wygenerować.
// rozwiazane przez 1.1.A