fn running_sum(nums: &[i32]) -> Result<Vec<i32>, &str> {
    if nums.is_empty() || nums.len() > 1000 {
        return Err("Input array *nums* should have a length between 1 and 1000.");
    }

    if !nums
        .iter()
        .all(|&num| (-1_000_000..=1_000_000).contains(&num))
    {
        return Err("All elements must be between -1,000,000 and 1,000,000.");
    }

    let result = nums
        .iter()
        .scan(0, |sum, &num| {
            *sum += num;
            Some(*sum)
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1, 2, 3, 4];
        let result = running_sum(&nums);
        assert_eq!(result, Ok(vec![1, 3, 6, 10]));
    }

    #[test]
    fn test_2() {
        let nums = [1, 1, 1, 1, 1];
        let result = running_sum(&nums);
        assert_eq!(result, Ok(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_3() {
        let nums = [3, 1, 2, 10, 1];
        let result = running_sum(&nums);
        assert_eq!(result, Ok(vec![3, 4, 6, 16, 17]));
    }
}
