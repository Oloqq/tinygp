use self::bench_1_1::*;

mod bench_1_1;

pub fn run_benchmark(selector: &str, seed: Option<u64>) {
    match selector {
        "1_1_a" => {
            bench_1_1_a(seed);
        },
        _ => {
            println!("Could not find the benchmark");
        }
    }
}