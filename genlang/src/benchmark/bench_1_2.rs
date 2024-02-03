use super::util::execute_benchmark;
use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::fitness_funcs::*;
use crate::Args;

// 1.2.A Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę.
//  Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [0,9]
pub fn bench_1_2_a(args: &Args) {
    let params = Params {
        popsize: 1000,
        random_initial_memory: true,
        acceptable_error: -0.1,
        max_size: 12,
        p_mut_per_node: 0.9,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.9,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![4, 7], vec![11]),
        (vec![4, 4], vec![8]),
        (vec![7, 8], vec![15]),
    ];

    execute_benchmark(args, params, cases, "1_2_a", diff_only);
}

// 1.2.B Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę. Na wejściu mogą być tylko całkowite liczby w zakresie [-9,9]
pub fn bench_1_2_b(args: &Args) {
    let params = Params {
        popsize: 1000,
        // random_initial_memory: true,
        max_size: 12,
        memsize: 3,
        acceptable_error: -0.1,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.9,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-9, 9], vec![0]),
        (vec![-5, 2], vec![-3]),
        (vec![5, -2], vec![3]),
        (vec![4, 7], vec![11]),
        (vec![4, 4], vec![8]),
        (vec![-7, -8], vec![-15]),
    ];

    execute_benchmark(args, params, cases, "1_2_b", diff_only);
}

// 1.2.C Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich sumę. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]
pub fn bench_1_2_c(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 12,
        // random_initial_memory: true,
        tournament_size: 5,
        memsize: 3,
        acceptable_error: -0.1,
        p_mut_per_node: 0.5,
        growing: GrowingParams {
            p_prefer_reg_over_num: 0.9,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![2000, 9999], vec![11999]),
        (vec![1, 1000], vec![1001]),
        (vec![-30, 230], vec![200]),
        (vec![-40, -1000], vec![-1040]),
        (vec![1, 3], vec![4]),
        (vec![-320, -430], vec![-750]),
        (vec![320, 320], vec![0]),
        (vec![-10, -10], vec![-20]),
        (vec![35, 35], vec![70]),
        (vec![1234, 1003], vec![2237]),
        (vec![5555, 1234], vec![6789]),
        (vec![0, 4123], vec![4123]),
        (vec![7890, -3100], vec![4790]),
        (vec![9999, 0], vec![9999]),
    ];

    execute_benchmark(args, params, cases, "1_2_c", diff_only);
}

// 1.2.D Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich różnicę. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]
pub fn bench_1_2_d(args: &Args) {
    let params = Params {
        popsize: 10000,
        random_initial_memory: true,
        max_size: 12,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![2000, 9999], vec![-7999]),
        (vec![5000, 2500], vec![2500]),
        (vec![1, 1000], vec![-999]),
    ];

    execute_benchmark(args, params, cases, "1_2_d", diff_only);
}

// 1.2.E Program powinien odczytać dwie pierwsze liczy z wejścia i zwrócić na wyjściu (jedynie) ich iloczyn. Na wejściu mogą być tylko całkowite liczby dodatnie w zakresie [-9999,9999]
pub fn bench_1_2_e(args: &Args) {
    let params = Params {
        popsize: 10000,
        random_initial_memory: true,
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![5, 500], vec![2500]),
        (vec![8, 80], vec![640]),
        (vec![100, 800], vec![80000]),
    ];

    execute_benchmark(args, params, cases, "1_2_e", diff_only);
}
