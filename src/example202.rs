fn permute_array(nums: &[u32]) -> Result<Vec<u32>, &str> {
    let n = u32::try_from(nums.len()).expect("Size of nums should fit in u32");

    if n == 0 || n > 1000 {
        return Err("Input array must have a length between 1 and 1000.");
    }

    let permuted = nums.iter().filter(|&item| item < &n).map(|&item| {
        nums.get(item as usize)
            .expect("Assume correct index")
            .to_owned()
    });

    let ans = permuted.collect::<Vec<u32>>();
    Ok(ans)
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
