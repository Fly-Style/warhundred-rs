use rstest::rstest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        // Arrange
        let a = 2;
        let b = 3;

        // Act
        let result = a + b;

        // Assert
        assert_eq!(result, 5);
    }

    #[rstest]
    #[case(1, 1, 2)]
    #[case(5, 7, 12)]
    #[case(10, -5, 5)]
    fn test_parameterized_addition(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        // Act
        let result = a + b;

        // Assert
        assert_eq!(result, expected);
    }
}
