use super::{common::*, execution::{Runtime, execute}};

const INPUT: Token = Token::Stat(Stat::INPUT);
const OUTPUT: Token = Token::Stat(Stat::OUTPUT);

use pretty_assertions::assert_eq;

fn run_cases(program: &Program, memsize: usize, cases: Vec<(Vec<f32>, Vec<f32>)>) {
    let _ = env_logger::builder().is_test(true).try_init();
    for (i, (input, expected_output)) in cases.into_iter().enumerate() {
        let runtime = Runtime::new(memsize, input);
        let output = execute(&program, runtime);
        println!("\nCase {i}");
        assert_eq!(output, expected_output);
    }
}

#[test]
fn test_identity() {
    let memsize = 3;
    let program = vec![INPUT, Token::Reg(0), OUTPUT, Token::Reg(0)];
    let cases: Vec<(Vec<f32>, Vec<f32>)> = vec![(vec![1.0], vec![1.0]), (vec![2.0], vec![2.0])];
    run_cases(&program, memsize, cases);
}
