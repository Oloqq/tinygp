use crate::params::{Case, Params, GrowingParams};
use crate::tinygp::common::{Expr, Stat};
use crate::tinygp::fitness_funcs::*;
use crate::Args;
use super::util::execute_benchmark;

// 1.4.A Program powinien odczytać dziesięć pierwszych liczy z wejścia i zwrócić na wyjściu (jedynie) ich średnią arytmetyczną (zaokrągloną do pełnej liczby całkowitej). Na wejściu mogą być tylko całkowite liczby w zakresie [-99,99]
pub fn bench_1_4_a(args: &Args) {
    let params = Params {
        popsize: 1000,
        max_size: 10000,
        growing: GrowingParams {
            d_expr: vec![
                (Expr::ADD, 4),
                (Expr::SUB, 0),
                (Expr::MUL, 0),
                (Expr::DIV, 1),
                (Expr::EQ, 0),
                (Expr::LT, 0),
                (Expr::GT, 0),
                (Expr::OR, 0),
                (Expr::AND, 0),
                (Expr::NOT, 0),
                (Expr::Num(0 as i32), 1),
                (Expr::Reg(0), 1),
            ],
            d_stat: vec![
                (Stat::LOAD, 1),
                (Stat::IF, 0),
                (Stat::WHILE, 0),
                (Stat::INPUT, 4),
                (Stat::OUTPUT, 1)
            ],
            p_prefer_reg_over_num: 0.8,
            min_const: 1,
            max_const: 11,
            p_insertion: 0.5,
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![78, 71, 61], vec![70]),
        (vec![34, 36, 69], vec![46]),
        (vec![-44, -80, -48], vec![-58]),
        (vec![-72, 54, -73], vec![-31]),
        (vec![65, -42, -72], vec![-17]),
        (vec![-67, -63, 94], vec![-12]),
        (vec![-95, 7, -54], vec![-48]),
        (vec![23, -29, -35], vec![-14]),
        (vec![82, -51, -72], vec![-14]),
        (vec![23, -32, 45], vec![12]),
    ];

    execute_benchmark(args, params, cases, "1_4_a", diff_first_promote_single);
}



// 1.4.B Program powinien odczytać na początek z wejścia pierwszą liczbę (ma być to wartość nieujemna)
//  a następnie tyle liczb (całkowitych) jaka jest wartość pierwszej odczytanej liczby
//  i zwrócić na wyjściu (jedynie) ich średnią arytmetyczną zaokrągloną do pełnej liczby całkowitej
// (do średniej nie jest wliczana pierwsza odczytana liczba, która mówi z ilu liczb chcemy obliczyć średnią).
//  Na wejściu mogą być tylko całkowite liczby w zakresie [-99,99], pierwsza liczba może być tylko w zakresie [0,99].
pub fn bench_1_4_b(_args: &Args) {
    todo!();
    // let params = Params {
    //     popsize: 10000,
    //     ..Default::default()
    // };
    // let cases: Vec<Case> = vec![
    //     (vec![0, 9], vec![9]),
    //     (vec![7, 1], vec![7]),
    //     (vec![2, 4], vec![4]),
    //     (vec![6, 0], vec![6]),
    // ];

    // execute_benchmark(args, params, cases, "1_4_b", diff_first_promote_single);
}
