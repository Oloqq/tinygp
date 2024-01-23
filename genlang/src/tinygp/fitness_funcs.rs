use super::common::Number;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub type FitnessFunc = fn(expected: &Vec<Number>, actual: &Vec<Number>) -> f32;

pub fn diff_first(expected: &Vec<Number>, actual: &Vec<Number>) -> f32 {
    let output = actual.get(0).unwrap_or(&0);
    let expected = expected[0];
    let error = (output - expected).abs();
    -error as f32
}

pub static FITNESS_FUNCS: Lazy<HashMap<String, FitnessFunc>> = Lazy::new(|| {
    let mut h: HashMap<String, FitnessFunc> = HashMap::new();
    h.insert("diff_first".into(), diff_first);
    h
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_first() {
        let expected = vec![1, 2, 43];
        assert_eq!(diff_first(&expected, &vec![1, 0, 12512, 453333]), 0.0);
        assert_eq!(diff_first(&expected, &vec![-1, 0, 12512, 453333]), -2.0);
    }
}