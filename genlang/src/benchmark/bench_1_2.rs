use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::fitness_funcs::*;
use crate::Args;
use super::util::execute_benchmark;

// 1.2.A Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę.
//  Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [0,9]
// TODO: implement terminating on reaching set output vector length, or another fitness accounting for the size of output vector
pub fn bench_1_2_a(args: &Args) {
    let params = Params {
        memsize: 3,
        popsize: 100,
        max_size: 4,
        p_crossover: 0.9,
        p_mut_per_node: 0.2,
        tournament_size: 2,
        growing: GrowingParams {
            p_prefer_reg_over_num: 1.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![0, 9], vec![9]),
        (vec![1, 4], vec![5]),
        (vec![7, 8], vec![15]),
    ];

    execute_benchmark(args, params, cases, "1_2_a", diff_first);
}


// 1.2.B Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę. Na wejściu mogą być tylko całkowite liczby w zakresie [-9,9]

// 1.2.C Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]

// 1.2.D Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich różnicę. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]

// 1.2.E Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich iloczyn. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]