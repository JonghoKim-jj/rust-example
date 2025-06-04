fn create_target_array(nums: &[u32], index: &[usize]) -> Result<Vec<u32>, &'static str> {
    if !(1..=100).contains(&nums.len()) {
        return Err("Length of *nums* and must be between 1 and 100");
    }
    if !(1..=100).contains(&index.len()) {
        return Err("Length of *index* and must be between 1 and 100");
    }

    if nums.len() != index.len() {
        return Err("Length of *nums* and *index* must be equal");
    }

    if !nums.iter().all(|&item| item <= 100_u32) {
        return Err("All item in *nums* must be between 0 and 100");
    }

    // Checking this assumption makes it safe to insert to target array using index
    if !index.iter().enumerate().all(|(idx, &item)| item <= idx) {
        return Err("For all i-th item in *index*, item must be equal or less than i");
    }

    let mut enumed: Vec<_> = index
        .iter()
        .zip(nums.iter())
        .enumerate()
        .map(|(i, (&idx, &item))| (idx, i, item))
        .collect();

    enumed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1).reverse()));

    let target = enumed.iter().map(|(_, _, item)| *item).collect();

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [0, 1, 2, 3, 4];
        let index = [0, 1, 2, 2, 1];
        let result = create_target_array(&nums, &index);

        assert_eq!(result, Ok(vec![0, 4, 1, 3, 2]));
    }

    #[test]
    fn test_2() {
        let nums = [1, 2, 3, 4, 0];
        let index = [0, 1, 2, 3, 0];
        let result = create_target_array(&nums, &index);

        assert_eq!(result, Ok(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_3() {
        let nums = [1];
        let index = [0];
        let result = create_target_array(&nums, &index);

        assert_eq!(result, Ok(vec![1]));
    }

    #[test]
    fn test_4() {
        let nums1 = [1; 100];
        let index1: Vec<usize> = (0_usize..=99).collect();
        let result1 = create_target_array(&nums1, &index1);

        let nums2 = [1; 101];
        let index2: Vec<usize> = (0_usize..=100).collect();
        let result2 = create_target_array(&nums2, &index2);

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
}
