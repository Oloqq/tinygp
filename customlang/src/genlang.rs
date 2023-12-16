#[allow(unused)]

fn tokenize(program: &str) -> Vec<&str> {
    program
        .split(' ')
        .filter(|t| t.len() > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("ADD 2 3"), ["ADD", "2", "3"]);
        assert_eq!(tokenize("  ADD  2 3  "), ["ADD", "2", "3"]);
    }
}
