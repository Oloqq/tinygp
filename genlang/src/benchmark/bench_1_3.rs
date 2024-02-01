use crate::params::{Case, Params};
use crate::tinygp::fitness_funcs::*;
use crate::Args;
use super::util::execute_benchmark;

// 1.3.A Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) większą z nich. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [0,9]
pub fn bench_1_3_a(args: &Args) {
    let params = Params {
        popsize: 10000,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0, 9], vec![9]),
        (vec![7, 1], vec![7]),
        (vec![2, 4], vec![4]),
        (vec![6, 0], vec![6]),
    ];

    execute_benchmark(args, params, cases, "1_3_a", diff_first_promote_single);
}


// 1.3.B Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) większą z nich. Na wejściu mogą być tylko całkowite liczby w zakresie [-9999,9999]
pub fn bench_1_3_b(args: &Args) {
    let params = Params {
        popsize: 10000,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-9999, 9999], vec![9999]),
        (vec![7777, 21], vec![7777]),
        (vec![2115, -600], vec![2115]),
        (vec![0, 15], vec![15]),
        (vec![15, 0], vec![15]),
        (vec![2, 3], vec![3]),
        (vec![3, 2], vec![3]),

    ];

    execute_benchmark(args, params, cases, "1_3_b", diff_first_promote_single);
}