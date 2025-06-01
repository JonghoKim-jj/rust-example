fn shuffle_array(nums: &[u32]) -> Result<Vec<u32>, &str> {
    let double_n = nums.len();
    if double_n % 2 != 0 {
        return Err("Input array must have an even length.");
    }

    let n = double_n / 2;

    if n == 0 || n > 500 {
        return Err("n must be between 1 and 500.");
    }

    if !nums.iter().all(|&item| (1_u32..=1_000).contains(&item)) {
        return Err("All elements in nums must be between 1 and 1000.");
    }

    let shuffled: Vec<u32> = nums
        .iter()
        .take(n)
        .zip(nums.iter().skip(n))
        .flat_map(|(&first, &second)| [first, second])
        .collect();

    Ok(shuffled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [2, 5, 1, 3, 4, 7];
        let result = shuffle_array(&nums);
        assert_eq!(result, Ok(vec![2, 3, 5, 4, 1, 7]));
    }

    #[test]
    fn test_2() {
        let nums = [1, 2, 3, 4, 4, 3, 2, 1];
        let result = shuffle_array(&nums);
        assert_eq!(result, Ok(vec![1, 4, 2, 3, 3, 2, 4, 1]));
    }

    #[test]
    fn test_3() {
        let nums = [1, 1, 2, 2];
        let result = shuffle_array(&nums);
        assert_eq!(result, Ok(vec![1, 2, 1, 2]));
    }

    #[test]
    fn test_4() {
        let nums = [];
        let result = shuffle_array(&nums);
        assert!(result.is_err());
    }

    #[test]
    fn test_5() {
        let nums = [1, 2, 3];
        let result = shuffle_array(&nums);
        assert!(result.is_err());
    }

    #[test]
    fn test_6() {
        let nums = [0, 1, 2, 3];
        let result = shuffle_array(&nums);
        assert!(result.is_err());
    }

    #[test]
    fn test_7() {
        let nums = [1, 1, 1, 1_000, 1_000, 1_000];
        let result = shuffle_array(&nums);
        assert_eq!(result, Ok(vec![1, 1_000, 1, 1_000, 1, 1_000]));
    }

    #[test]
    fn test_8() {
        let nums = [1, 1, 1, 1_001, 1_001, 1_001];
        let result = shuffle_array(&nums);
        assert!(result.is_err());
    }
}
