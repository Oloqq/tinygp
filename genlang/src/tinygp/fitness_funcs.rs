use super::common::{Number, Program};
use once_cell::sync::Lazy;
use std::{cmp, collections::HashMap};

pub type FitnessFunc = fn(expected: &Vec<Number>, actual: &Vec<Number>) -> f32;

pub fn diff_first(expected: &Vec<Number>, actual: &Vec<Number>) -> f32 {
    let output = actual.get(0).unwrap_or(&0);
    let expected = expected[0];
    let error = (*output as f32 - expected as f32).abs();
    -error
}

pub fn diff_best(expected: &Vec<Number>, actual: &Vec<Number>) -> f32 {
    let mut min_error = f32::MAX;
    for i in 0..cmp::min(expected.len(), actual.len()) {
        let output = actual.get(i).unwrap_or(&0);
        let expected = expected[i];
        let error = (*output as f32 - expected as f32).abs();
        if error < min_error {
            min_error = error;
        }
    }
    -min_error
}

pub fn diff_only(expected: &Vec<Number>, actual: &Vec<Number>) -> f32 {
    if actual.len() != 1 {
        return f32::MIN;
    }
    return diff_first(expected, actual);
}

pub fn diff_first_promote_single(expected: &Vec<Number>, actual: &Vec<Number>) -> f32 {
    let output = match actual.get(0) {
        Some(x) => x,
        None => return -f32::INFINITY,
    };
    let expected = expected[0];
    let error = (*output as f32 - expected as f32).abs();
    -error * f32::sqrt(actual.len() as f32)
}

pub fn fit_arithmetic_series(expected: &Vec<Number>, actual: &Vec<Number>) -> f32 {
    let mut error: f32 = 0.0;
    for i in 0..cmp::min(expected.len(), actual.len()) {
        let output = match actual.get(i) {
            Some(x) => x,
            None => return f32::MIN,
        };
        error += (*output as f32 - expected[i] as f32).abs();
    }
    error += 10000.0 * (expected.len() as f32 - actual.len() as f32).abs();
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
        assert_eq!(diff_first(&expected, &vec![1, 0, 12512, 453333]), 0.0);
        assert_eq!(diff_first(&expected, &vec![-1, 0, 12512, 453333]), -2.0);
    }

    // #[test]
    // fn test_diff_first() {
    //     let expected = vec![1, 2, 43];
    //     assert_eq!(diff_first(&expected, &vec![1, 0, 12512, 453333]), 0.0);
    //     assert_eq!(diff_first(&expected, &vec![-1, 0, 12512, 453333]), -2.0);
    // }
}
