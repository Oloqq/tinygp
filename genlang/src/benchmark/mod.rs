use crate::Args;

mod bench_1_1;
mod bench_1_2;
mod bench_1_3;
mod bench_1_4;
mod bench_final;
mod bench_final_bool;
mod util;

use self::bench_1_1::*;
use self::bench_1_2::*;
use self::bench_1_3::*;
use self::bench_1_4::*;
use self::bench_final::*;
use self::bench_final_bool::*;

pub fn run_benchmark(suite: &str, args: &Args) {
    match suite {
        "1_1_a" => bench_1_1_a(args),
        "1_1_b" => bench_1_1_b(args),
        "1_1_c" => bench_1_1_c(args),
        "1_1_d" => bench_1_1_d(args),
        "1_1_e" => bench_1_1_e(args),
        "1_1_f" => bench_1_1_f(args),

        "1_2_a" => bench_1_2_a(args),
        "1_2_b" => bench_1_2_b(args),
        "1_2_c" => bench_1_2_c(args),
        "1_2_d" => bench_1_2_d(args),
        "1_2_e" => bench_1_2_e(args),

        "1_3_a" => bench_1_3_a(args),
        "1_3_b" => bench_1_3_b(args),

        "1_4_a" => bench_1_4_a(args),
        "1_4_b" => bench_1_4_b(args),

        "final_1" => bench_final_1(args),
        "final_2" => bench_final_2(args),
        "final_3" => bench_final_3(args),
        "final_bool_1" => bench_final_bool_1(args),
        "final_bool_2" => bench_final_bool_2(args),
        "final_bool_3" => bench_final_bool_3(args),
        "final_bool_4" => bench_final_bool_4(args),
        "final_bool_5" => bench_final_bool_5(args),
        "final_bool_6" => bench_final_bool_6(args),
        "final_bool_7" => bench_final_bool_7(args),
        "final_bool_8" => bench_final_bool_8(args),
        "final_bool_9" => bench_final_bool_9(args),
        "final_bool_10" => bench_final_bool_10(args),
        _ => {
            println!("Could not find the benchmark");
        }
    }
}
