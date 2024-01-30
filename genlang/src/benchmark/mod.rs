use crate::Args;

use self::bench_1_1::*;

mod util;
mod bench_1_1;

pub fn run_benchmark(suite: &str, args: &Args) {
    match suite {
        "1_1_a" => {
            bench_1_1_a(args);
        },
        "1_1_b" => {
            bench_1_1_b(args);
        },
        _ => {
            println!("Could not find the benchmark");
        }
    }
}