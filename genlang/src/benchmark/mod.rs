use self::bench_1_1::*;

mod bench_1_1;

pub fn run_benchmark(selector: &str) {
    match selector {
        "1_1_a" => {
            bench_1_1_a();
        },
        _ => {
            println!("Could not find a benchmark");
        }
    }
}