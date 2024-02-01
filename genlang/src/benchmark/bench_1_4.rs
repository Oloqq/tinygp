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
        memsize: 15,
        growing: GrowingParams {
            d_expr: vec![
                (Expr::ADD, 4),
                (Expr::SUB, 1),
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
                (Stat::WHILE, 1),
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
        (vec![32, -80, 48], vec![0]),
        (vec![74, -58, 74], vec![30]),
        (vec![40, 84, -59], vec![21]),
        (vec![59, 71, 20], vec![50]),
        (vec![-94, -89, 21], vec![-54]),
        (vec![-41, -26, 9], vec![-20]),
        (vec![10, -35, 66], vec![13]),
        (vec![40, 47, -18], vec![23]),
        (vec![69, 25, 82], vec![58]),
        (vec![-54, -94, -64], vec![-71]),
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
