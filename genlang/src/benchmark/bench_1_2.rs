use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::fitness_funcs::*;
use crate::Args;
use super::util::execute_benchmark;

// 1.2.A Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę.
//  Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [0,9]
// TODO: implement terminating on reaching set output vector length, or another fitness accounting for the size of output vector
pub fn bench_1_2_a(args: &Args) {
    let params = Params {
        popsize: 10000,
        random_initial_memory: true,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![4, 7], vec![11]),
        (vec![4, 4], vec![8]),
        (vec![7, 8], vec![15]),
    ];

    execute_benchmark(args, params, cases, "1_2_a", diff_first);
}


// 1.2.B Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę. Na wejściu mogą być tylko całkowite liczby w zakresie [-9,9]


// 1.2.C Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]

// 1.2.D Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich różnicę. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]

// 1.2.E Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich iloczyn. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]
pub fn bench_1_2_e(args: &Args) {
    let params = Params {
        popsize: 10000,
        random_initial_memory: true,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-9999, 9999], vec![0]),
        (vec![-5000, 2500], vec![-2500]),
        (vec![5000, -2500], vec![2500]),
    ];

    execute_benchmark(args, params, cases, "1_2_e", diff_first);
}
