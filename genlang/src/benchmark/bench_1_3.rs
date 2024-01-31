use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::fitness_funcs::*;
use crate::Args;
use super::util::execute_benchmark;

// 1.3.A Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) większą z nich. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [0,9]
pub fn bench_1_3_a(args: &Args) {
    let params = Params {
        popsize: 10000,
        random_initial_memory: true,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![2000, 9999], vec![11999]),
        (vec![5000, 2500], vec![7500]),
        (vec![1, 1000], vec![1001]),
    ];

    execute_benchmark(args, params, cases, "1_3_a", diff_first);
}


// 1.3.B Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) większą z nich. Na wejściu mogą być tylko całkowite liczby w zakresie [-9999,9999]