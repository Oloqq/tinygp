use super::util::execute_benchmark;
use crate::params::{Case, GrowingParams, Params};
use crate::tinygp::common::{Expr, Stat, Token};
use crate::tinygp::fitness_funcs::*;
use crate::Args;

// 3. - Given 3 integers start, end and step, print the integers in the arithemtic series
pub fn bench_final_1(args: &Args) {
    let params = Params {
        popsize: 5000,
        max_size: 200,
        memsize: 20,
        prefix: vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(0),
            Token::Stat(Stat::INPUT),
            Token::Reg(1),
            Token::Stat(Stat::INPUT),
            Token::Reg(2),
        ],
        suffix: vec![],
        growing: GrowingParams {
            d_expr: vec![
                (Expr::ADD, 1),
                (Expr::SUB, 0),
                (Expr::MUL, 0),
                (Expr::DIV, 0),
                (Expr::EQ, 0),
                (Expr::LT, 1),
                (Expr::GT, 1),
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
                (Stat::INPUT, 1),
                (Stat::OUTPUT, 1),
            ],
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-12, 66, 15], vec![-12, 3, 18, 33, 48, 63]),
        (vec![38, -1, -15], vec![38, 23, 8]),
        (vec![51, 55, 12], vec![51]),
        (vec![-2, 5, 4], vec![-2, 2]),
        (
            vec![12, 28, 1],
            vec![
                12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
            ],
        ),
        (vec![-90, -91, -3], vec![-90]),
        (
            vec![-82, 68, 12],
            vec![-82, -70, -58, -46, -34, -22, -10, 2, 14, 26, 38, 50, 62],
        ),
        (vec![-16, -59, -13], vec![-16, -29, -42, -55]),
        (vec![81, 7, -18], vec![81, 63, 45, 27, 9]),
        (vec![76, 76, 2], vec![]),
        (
            vec![-20, 74, 10],
            vec![-20, -10, 0, 10, 20, 30, 40, 50, 60, 70],
        ),
        (
            vec![12, -75, -6],
            vec![
                12, 6, 0, -6, -12, -18, -24, -30, -36, -42, -48, -54, -60, -66, -72,
            ],
        ),
        (vec![-81, -41, 18], vec![-81, -63, -45]),
        (vec![84, 34, -11], vec![84, 73, 62, 51, 40]),
        (vec![4, -84, -17], vec![4, -13, -30, -47, -64, -81]),
        (vec![22, 89, 16], vec![22, 38, 54, 70, 86]),
        (vec![37, 3, -10], vec![37, 27, 17, 7]),
        (vec![93, 55, -17], vec![93, 76, 59]),
        (vec![-78, -1, 20], vec![-78, -58, -38, -18]),
        (vec![-38, -68, -11], vec![-38, -49, -60]),
        (vec![-34, -87, -10], vec![-34, -44, -54, -64, -74, -84]),
        (vec![26, -20, -16], vec![26, 10, -6]),
        (vec![47, -19, -10], vec![47, 37, 27, 17, 7, -3, -13]),
        (
            vec![-25, 78, 9],
            vec![-25, -16, -7, 2, 11, 20, 29, 38, 47, 56, 65, 74],
        ),
        (
            vec![53, -1, -3],
            vec![
                53, 50, 47, 44, 41, 38, 35, 32, 29, 26, 23, 20, 17, 14, 11, 8, 5, 2,
            ],
        ),
        (vec![18, 76, 11], vec![18, 29, 40, 51, 62, 73]),
        (vec![86, 98, 7], vec![86, 93]),
        (
            vec![-56, 20, 7],
            vec![-56, -49, -42, -35, -28, -21, -14, -7, 0, 7, 14],
        ),
        (vec![94, 95, 13], vec![94]),
        (vec![63, 64, 1], vec![63]),
        (
            vec![-33, 97, 3],
            vec![
                -33, -30, -27, -24, -21, -18, -15, -12, -9, -6, -3, 0, 3, 6, 9, 12, 15, 18, 21, 24,
                27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60, 63, 66, 69, 72, 75, 78, 81, 84, 87,
                90, 93, 96,
            ],
        ),
        (
            vec![73, -23, -3],
            vec![
                73, 70, 67, 64, 61, 58, 55, 52, 49, 46, 43, 40, 37, 34, 31, 28, 25, 22, 19, 16, 13,
                10, 7, 4, 1, -2, -5, -8, -11, -14, -17, -20,
            ],
        ),
        (vec![-14, -16, -15], vec![-14]),
        (vec![51, 77, 10], vec![51, 61, 71]),
        (
            vec![49, -18, -1],
            vec![
                49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29,
                28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8,
                7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13,
                -14, -15, -16, -17,
            ],
        ),
        (vec![-23, 25, 20], vec![-23, -3, 17]),
        (
            vec![-5, 38, 4],
            vec![-5, -1, 3, 7, 11, 15, 19, 23, 27, 31, 35],
        ),
        (
            vec![-71, -88, -1],
            vec![
                -71, -72, -73, -74, -75, -76, -77, -78, -79, -80, -81, -82, -83, -84, -85, -86, -87,
            ],
        ),
        (vec![-26, -61, -13], vec![-26, -39, -52]),
        (
            vec![43, 71, 1],
            vec![
                43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
                64, 65, 66, 67, 68, 69, 70,
            ],
        ),
        (vec![-11, -71, -19], vec![-11, -30, -49, -68]),
        (vec![-77, -66, 4], vec![-77, -73, -69]),
        (vec![31, 66, 16], vec![31, 47, 63]),
        (vec![-47, -68, -5], vec![-47, -52, -57, -62, -67]),
        (
            vec![32, 85, 1],
            vec![
                32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
                74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84,
            ],
        ),
        (vec![45, 26, -4], vec![45, 41, 37, 33, 29]),
        (
            vec![-84, 22, 8],
            vec![
                -84, -76, -68, -60, -52, -44, -36, -28, -20, -12, -4, 4, 12, 20,
            ],
        ),
        (vec![34, 44, 12], vec![34]),
        (vec![-65, -60, 7], vec![-65]),
        (vec![38, 57, 15], vec![38, 53]),
    ];

    execute_benchmark(args, params, cases, "final_1", fit_arithmetic_series);
}

fn sum_of_squares(to: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..to + 1 {
        sum += i * i;
    }
    sum
}

fn sum(to: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..to + 1 {
        sum += i;
    }
    sum
}

pub fn bench_sum(args: &Args) {
    let params = Params {
        popsize: 2000,
        max_size: 100,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1], vec![sum(1)]),
        (vec![2], vec![sum(2)]),
        (vec![3], vec![sum(3)]),
        (vec![4], vec![sum(4)]),
        (vec![5], vec![sum(5)]),
        (vec![6], vec![sum(6)]),
        (vec![7], vec![sum(7)]),
        (vec![8], vec![sum(8)]),
        (vec![9], vec![sum(9)]),
        (vec![10], vec![sum(10)]),
    ];

    execute_benchmark(
        args,
        params,
        cases,
        "final_2_sum",
        diff_first_promote_single,
    );
}

pub fn bench_square(args: &Args) {
    let params = Params {
        popsize: 2000,
        max_size: 100,
        memsize: 1,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1], vec![1 * 1]),
        (vec![2], vec![2 * 2]),
        (vec![3], vec![3 * 3]),
        (vec![4], vec![4 * 4]),
        (vec![5], vec![5 * 5]),
        (vec![6], vec![6 * 6]),
        (vec![7], vec![7 * 7]),
        (vec![8], vec![8 * 8]),
        (vec![9], vec![9 * 9]),
        (vec![10], vec![10 * 10]),
    ];

    execute_benchmark(
        args,
        params,
        cases,
        "final_2_square",
        diff_first_promote_single,
    );
}

// 17. - Given integer n, return the sum of squaring each integer in the range [1, n]
pub fn bench_final_2(args: &Args) {
    let params = Params {
        popsize: 5000,
        max_size: 200,
        memsize: 15,
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![1], vec![sum_of_squares(1)]),
        (vec![2], vec![sum_of_squares(2)]),
        (vec![3], vec![sum_of_squares(3)]),
        (vec![4], vec![sum_of_squares(4)]),
        (vec![5], vec![sum_of_squares(5)]),
        (vec![6], vec![sum_of_squares(6)]),
        (vec![7], vec![sum_of_squares(7)]),
        (vec![8], vec![sum_of_squares(8)]),
        (vec![9], vec![sum_of_squares(9)]),
        (vec![10], vec![sum_of_squares(10)]),
    ];

    execute_benchmark(args, params, cases, "final_2", diff_first_promote_single);
}

// 28. - Print the smallest of 4 integers
pub fn bench_final_3(args: &Args) {
    let params = Params {
        popsize: 5000,
        max_size: 200,
        memsize: 4,
        acceptable_error: -50.0,
        prefix: vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(0),
            Token::Stat(Stat::INPUT),
            Token::Reg(1),
            Token::Stat(Stat::INPUT),
            Token::Reg(2),
            Token::Stat(Stat::INPUT),
            Token::Reg(3),
        ],
        suffix: vec![],
        growing: GrowingParams {
            ..Default::default()
        },
        ..Default::default()
    };
    let cases: Vec<Case> = vec![
        (vec![-36, -62, -94, 17], vec![-94]),
        (vec![74, 4, 58, -27], vec![-27]),
        (vec![90, 93, 52, 13], vec![13]),
        (vec![-39, 25, -60, -82], vec![-82]),
        (vec![-22, 83, -55, 55], vec![-55]),
        (vec![49, -51, 51, -62], vec![-62]),
        (vec![-97, -24, 57, 9], vec![-97]),
        (vec![-96, -2, 53, 65], vec![-96]),
        (vec![-18, 80, -62, -96], vec![-96]),
        (vec![77, -18, 81, -91], vec![-91]),
        (vec![53, 21, 48, -45], vec![-45]),
        (vec![-96, 10, 6, 21], vec![-96]),
        (vec![-54, -41, -71, -99], vec![-99]),
        (vec![-96, -98, -69, 20], vec![-98]),
        (vec![97, -5, 81, 18], vec![-5]),
        (vec![-82, -78, -24, -55], vec![-82]),
        (vec![42, -51, -77, 53], vec![-77]),
        (vec![98, 57, -51, 94], vec![-51]),
        (vec![-47, -38, -15, -42], vec![-47]),
        (vec![89, 95, 53, 48], vec![48]),
        (vec![11, -18, 84, -45], vec![-45]),
        (vec![-6, -16, -18, 61], vec![-18]),
        (vec![-1, 28, 27, 49], vec![-1]),
        (vec![63, 39, -50, -82], vec![-82]),
        (vec![32, -46, -100, -18], vec![-100]),
        (vec![58, 99, -51, 52], vec![-51]),
        (vec![-9, -5, -49, 43], vec![-49]),
        (vec![-93, 31, -45, 75], vec![-93]),
        (vec![-16, 69, 59, -60], vec![-60]),
        (vec![-83, -29, 29, -8], vec![-83]),
        (vec![-63, -60, -24, 87], vec![-63]),
        (vec![-48, -71, -81, -25], vec![-81]),
        (vec![-29, 71, 68, 67], vec![-29]),
        (vec![22, -85, 44, -77], vec![-85]),
        (vec![20, 90, 32, 77], vec![20]),
        (vec![87, 13, -94, 59], vec![-94]),
        (vec![-32, 30, -37, 35], vec![-37]),
        (vec![-87, 18, 30, 87], vec![-87]),
        (vec![-63, 85, -46, -41], vec![-63]),
        (vec![52, -23, 24, -61], vec![-61]),
        (vec![-86, 76, 78, 44], vec![-86]),
        (vec![-92, 20, 49, -15], vec![-92]),
        (vec![37, -25, -68, 96], vec![-68]),
        (vec![-40, 43, -39, -45], vec![-45]),
        (vec![30, -13, -80, 93], vec![-80]),
        (vec![-13, 62, 83, -42], vec![-42]),
        (vec![31, -53, -65, 80], vec![-65]),
        (vec![-61, -65, 41, 17], vec![-65]),
        (vec![-73, -77, -98, 81], vec![-98]),
        (vec![74, 38, 46, -13], vec![-13]),
        (vec![-68, 28, -41, 0], vec![-68]),
        (vec![6, -64, -18, 24], vec![-64]),
        (vec![-78, -66, -41, -58], vec![-78]),
        (vec![-96, -95, 25, 4], vec![-96]),
        (vec![35, -61, -35, 22], vec![-61]),
        (vec![-94, -92, 40, 49], vec![-94]),
        (vec![-49, 27, 97, 59], vec![-49]),
        (vec![-55, -69, 51, 27], vec![-69]),
        (vec![-2, 73, 98, -40], vec![-40]),
        (vec![85, 86, 90, -54], vec![-54]),
        (vec![48, 99, -33, -8], vec![-33]),
        (vec![-67, -41, -9, -59], vec![-67]),
        (vec![-95, 50, -11, -65], vec![-95]),
        (vec![43, 61, -8, -10], vec![-10]),
        (vec![39, -7, 44, -83], vec![-83]),
        (vec![-53, -23, -43, -36], vec![-53]),
        (vec![-17, -29, -77, 41], vec![-77]),
        (vec![9, -83, 54, 1], vec![-83]),
        (vec![63, -35, -95, 10], vec![-95]),
        (vec![-67, -10, 79, 80], vec![-67]),
        (vec![-1, 50, 36, 27], vec![-1]),
        (vec![-51, 60, -60, -97], vec![-97]),
        (vec![62, 61, 71, 46], vec![46]),
        (vec![98, -33, 84, -13], vec![-33]),
        (vec![80, -68, 22, -49], vec![-68]),
        (vec![61, 23, -55, -21], vec![-55]),
        (vec![-74, 32, 2, 44], vec![-74]),
        (vec![0, -86, 28, 87], vec![-86]),
        (vec![-92, -20, -63, -14], vec![-92]),
        (vec![39, 92, -18, -44], vec![-44]),
        (vec![18, 70, -81, -100], vec![-100]),
        (vec![-11, -27, 40, 4], vec![-27]),
        (vec![-24, 76, 11, -96], vec![-96]),
        (vec![-91, -49, -41, 46], vec![-91]),
        (vec![67, 44, 5, -16], vec![-16]),
        (vec![66, 7, 31, 78], vec![7]),
        (vec![34, 62, 27, -40], vec![-40]),
        (vec![-49, -6, -39, -86], vec![-86]),
        (vec![27, -21, 20, 82], vec![-21]),
        (vec![-43, 64, 52, -62], vec![-62]),
        (vec![-65, -46, 77, -86], vec![-86]),
        (vec![83, -34, -31, -51], vec![-51]),
        (vec![-33, 98, 19, -29], vec![-33]),
        (vec![94, 0, 91, -61], vec![-61]),
        (vec![90, 28, -18, 52], vec![-18]),
        (vec![46, -31, -29, 98], vec![-31]),
        (vec![40, -8, 76, 31], vec![-8]),
        (vec![-25, -69, -2, -62], vec![-69]),
        (vec![-54, 39, -10, 12], vec![-54]),
        (vec![-85, 66, -97, -27], vec![-97]),
    ];

    execute_benchmark(args, params, cases, "final_3", diff_first_promote_single);
}
