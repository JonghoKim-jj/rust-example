fn find_arr(encoded: &[i32], first: i32) -> Result<Vec<i32>, &'static str> {
    if !(1..10_000).contains(&encoded.len()) {
        return Err("Length of *encoded* must be between 1 and 9,999");
    }

    if !(0..=100_000).contains(&first) {
        return Err("*first* must be between 0 and 100,000");
    }

    if !encoded.iter().all(|val| (0..=100_000).contains(val)) {
        return Err("All value must be between 0 and 100,000");
    }

    let arr: Vec<i32> = std::iter::once(first)
        .chain(encoded.iter().scan(first, |prev_item, item| {
            *prev_item ^= *item;
            Some(*prev_item)
        }))
        .collect();

    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let encoded = [1, 2, 3];
        let first = 1;
        let result = find_arr(&encoded, first);

        assert_eq!(result, Ok(vec![1, 0, 2, 1]));
    }

    #[test]
    fn test_2() {
        let encoded = [6, 2, 7, 3];
        let first = 4;
        let result = find_arr(&encoded, first);

        assert_eq!(result, Ok(vec![4, 2, 0, 7, 4]));
    }

    #[test]
    fn test_3() {
        let encoded = [1, -1];
        let first = 2;
        let result = find_arr(&encoded, first);

        assert!(result.is_err());
    }
}
