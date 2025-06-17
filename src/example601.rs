fn final_value(operations: &[&str]) -> Result<i32, &'static str> {
    if !(1..=100).contains(&operations.len()) {
        return Err("Length of *operations* must be between 1 and 100");
    }

    operations
        .iter()
        .try_fold(0, |x, &op| match op {
            "++X" | "X++" => Some(x + 1),
            "--X" | "X--" => Some(x - 1),
            _ => None,
        })
        .ok_or("Operation must be among ++X, X++, --X, X--")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let operations = vec!["--X", "X++", "X++"];
        let result = final_value(&operations);

        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_2() {
        let operations = vec!["++X", "++X", "X++"];
        let result = final_value(&operations);

        assert_eq!(result, Ok(3));
    }

    #[test]
    fn test_3() {
        let operations = vec!["X++", "++X", "--X", "X--"];
        let result = final_value(&operations);

        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_4() {
        let operations = vec![];
        let result = final_value(&operations);

        assert!(result.is_err());
    }

    #[test]
    fn test_5() {
        let operations1 = vec!["X++"; 100];
        let operations2 = vec!["X++"; 101];
        let result1 = final_value(&operations1);
        let result2 = final_value(&operations2);

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert_eq!(result1, Ok(100));
    }

    #[test]
    fn test_6() {
        let operations = vec!["Y++", "----X"];
        assert!(final_value(&operations).is_err());
    }
}
