fn permute_array(nums: &[u32]) -> Result<Vec<u32>, &str> {
    if !(1..=1_000).contains(&nums.len()) {
        return Err("Input array must have a length between 1 and 1000.");
    }

    let n = u32::try_from(nums.len()).expect("Already checked the length is between 1 and 1,000");

    // Check whether the elements in *nums* are distinct and in the range 0..n
    if !(0..n).all(|item| nums.contains(&item)) {
        return Err("elements in *nums* must be distinct and in the range 0..n");
    }

    let permuted = nums
        .iter()
        .map(|&item| {
            let idx = usize::try_from(item).expect("Already checked idx < n < 1000");
            nums.get(idx)
                .expect("Already checked idx < n == length")
                .to_owned()
        })
        .collect();

    Ok(permuted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [0, 2, 1, 5, 3, 4];
        let result = permute_array(&nums);
        assert_eq!(result, Ok(vec![0, 1, 2, 4, 5, 3]));
    }

    #[test]
    fn test_2() {
        let nums = [5, 0, 1, 2, 3, 4];
        let result = permute_array(&nums);
        assert_eq!(result, Ok(vec![4, 5, 0, 1, 2, 3]));
    }

    #[test]
    fn test_3() {
        let nums: Vec<u32> = (0_u32..=999).collect();
        let result = permute_array(&nums);
        assert_eq!(result, Ok(nums.clone()));
    }

    #[test]
    fn test_4() {
        let nums: Vec<u32> = (0_u32..=1000).collect();
        let result = permute_array(&nums);
        assert!(result.is_err());
    }

    #[test]
    fn test_5() {
        let empty_num: [u32; 0] = [];
        let result = permute_array(&empty_num);
        assert!(result.is_err());
    }
}
