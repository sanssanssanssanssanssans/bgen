#[cfg(test)]
mod tests {
    use crate::r#gen::generate;

    #[test]
    fn test_basic_hello() {
        let input = "A";
        let result = generate(input);
        assert_eq!(result.chars().filter(|&c| c == '+').count(), 65);
        assert!(result.ends_with('.'));
    }

    #[test]
    fn test_value_persistence() {
        let input = "AB";
        let result = generate(input);
        assert_eq!(result.chars().filter(|&c| c == '+').count(), 66);
        assert_eq!(result.chars().filter(|&c| c == '.').count(), 2);
    }
}
