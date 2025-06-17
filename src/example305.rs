fn solution(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    if !(2..=500).contains(&nums.len()) {
        return Err("Length of *num* must between 2 and 500");
    }

    let result = nums
        .iter()
        .map(|val| {
            i32::try_from(nums.iter().filter(|item| val > item).count())
                .expect("Length does not exceed 500, thus no overflow")
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![8, 1, 2, 2, 3];
        assert_eq!(solution(&nums), Ok(vec![4, 0, 1, 1, 3]));
    }

    #[test]
    fn test_2() {
        let nums = vec![6, 5, 4, 8];
        assert_eq!(solution(&nums), Ok(vec![2, 1, 0, 3]));
    }
    #[test]
    fn test_3() {
        let nums = vec![7, 7, 7, 7];
        assert_eq!(solution(&nums), Ok(vec![0, 0, 0, 0]));
    }
}
