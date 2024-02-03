use super::{
    common::{Number, Program},
    execution::Runtime,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type FitnessFunc = fn(expected: &Vec<Number>, actual: &Vec<Number>, runtime: &Runtime) -> f32;

pub fn diff_first(expected: &Vec<Number>, actual: &Vec<Number>, _runtime: &Runtime) -> f32 {
    let output = actual.get(0).unwrap_or(&0);
    let expected = expected[0];
    let error = (*output as f32 - expected as f32).abs();
    -error
}

pub fn diff_first_promote_single(
    expected: &Vec<Number>,
    actual: &Vec<Number>,
    _runtime: &Runtime,
) -> f32 {
    let output = match actual.get(0) {
        Some(x) => x,
        None => return -f32::INFINITY,
    };
    let expected = expected[0];
    let error = (*output as f32 - expected as f32).abs();
    -error * f32::sqrt(actual.len() as f32)
}

fn distance(a: Number, b: Number) -> f32 {
    a.abs_diff(b) as f32
}

fn distance_square(a: Number, b: Number) -> f32 {
    (a.abs_diff(b) as f32).powi(2)
}

fn promote_output_len(wanted: usize, actual: usize) -> f32 {
    actual.abs_diff(wanted) as f32 + 1.0
}

fn promote_reading(runtime: &Runtime) -> f32 {
    (runtime.input.len().abs_diff(runtime.input_cursor) + 1) as f32
}

fn punish_overreading(runtime: &Runtime) -> f32 {
    if runtime.overread { f32::MAX } else { 1.0 }
}

pub fn fit_exact_reading(expected: &Vec<Number>, actual: &Vec<Number>, runtime: &Runtime) -> f32 {
    -promote_reading(runtime) * punish_overreading(runtime)
}

pub fn diff_first_promote_reading(
    expected: &Vec<Number>,
    actual: &Vec<Number>,
    runtime: &Runtime,
) -> f32 {
    let output = match actual.get(0) {
        Some(x) => x,
        None => return -f32::INFINITY,
    };
    let expected = expected[0];
    let error = distance_square(*output, expected)
        * promote_output_len(1, actual.len())
        * promote_reading(runtime)
        * punish_overreading(runtime);
    -error
}

pub static FITNESS_FUNCS: Lazy<HashMap<String, FitnessFunc>> = Lazy::new(|| {
    let mut h: HashMap<String, FitnessFunc> = HashMap::new();
    h.insert("diff_first".into(), diff_first);
    h
});

pub fn normalize_fitness(fitness: &Vec<f32>, _programs: &Vec<Program>) -> Vec<f64> {
    // assert_eq!(fitness.len(), programs.len());
    // let fitness_max = fitness.iter().max_by(|a, b| a.total_cmp(b)).unwrap().clone() as f64;
    // let prog_len_max = programs.iter().map(|p| p.len()).max().unwrap().clone() as f64;
    // fitness.iter().zip(programs).map(|(fit, prog)| {
    //     (1.3 - (prog.len() as f64 / prog_len_max)) * (*fit as f64 / fitness_max)
    // }).collect()
    fitness.iter().map(|f| *f as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_first() {
        let expected = vec![1, 2, 43];
        let inp = vec![1];
        let runtime = Runtime::new(1, &inp, &mut None);
        assert_eq!(
            diff_first(&expected, &vec![1, 0, 12512, 453333], &runtime),
            0.0
        );
        assert_eq!(
            diff_first(&expected, &vec![-1, 0, 12512, 453333], &runtime),
            -2.0
        );
    }

    // #[test]
    // fn test_diff_first() {
    //     let expected = vec![1, 2, 43];
    //     assert_eq!(diff_first(&expected, &vec![1, 0, 12512, 453333]), 0.0);
    //     assert_eq!(diff_first(&expected, &vec![-1, 0, 12512, 453333]), -2.0);
    // }
}
