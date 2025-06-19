fn final_value(operations: &[&str]) -> Option<i32> {
    operations.iter().try_fold(0, |x, &op| match op {
        "++X" | "X++" => Some(x + 1),
        "--X" | "X--" => Some(x - 1),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let operations = vec!["--X", "X++", "X++"];
        let result = final_value(&operations);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_2() {
        let operations = vec!["++X", "++X", "X++"];
        let result = final_value(&operations);

        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_3() {
        let operations = vec!["X++", "++X", "--X", "X--"];
        let result = final_value(&operations);

        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_4() {
        let operations = vec![];
        let result = final_value(&operations);

        assert!(result.is_some_and(|x| x == 0));
    }

    #[test]
    fn test_5() {
        let operations1 = vec!["X++"; 100];
        let operations2 = vec!["X++"; 101];
        let result1 = final_value(&operations1);
        let result2 = final_value(&operations2);

        assert!(result1.is_some_and(|x| x == 100));
        assert!(result2.is_some_and(|x| x == 101));
    }

    #[test]
    fn test_6() {
        let operations = vec!["Y++", "----X"];
        assert!(final_value(&operations).is_none());
    }
}
