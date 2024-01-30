use crate::Args;

use self::bench_1_1::*;

mod bench_1_1;
mod bench_1_2;
mod bench_1_3;
mod bench_1_4;
mod util;

pub fn run_benchmark(suite: &str, args: &Args) {
    match suite {
        "1_1_a" => bench_1_1_a(args),
        "1_1_b" => bench_1_1_b(args),
        "1_1_c" => bench_1_1_c(args),

        // "1_2_a" => bench_1_2_a(args),
        // "1_2_b" => bench_1_2_b(args),
        // "1_2_c" => bench_1_2_c(args),
        // "1_2_d" => bench_1_2_d(args),
        // "1_2_e" => bench_1_2_e(args),

        // "1_3_a" => bench_1_3_a(args),
        // "1_3_b" => bench_1_3_b(args),
        // "1_4_a" => bench_1_4_a(args),
        // "1_4_b" => bench_1_4_b(args),
        _ => {
            println!("Could not find the benchmark");
        }
    }
}
