fn decompress(nums: &[u32]) -> Result<Vec<u32>, &'static str> {
    if !(2..=100).contains(&nums.len()) {
        return Err("Length of *nums* must be between 2 and 100");
    }

    if nums.len() % 2 == 1 {
        return Err("Length of *nums* must be even");
    }

    let result: Vec<u32> = nums
        .chunks_exact(2)
        .flat_map(|fvpair| {
            let freq = fvpair
                .first()
                .expect("At least there is one element in every chunks");
            let val = fvpair
                .get(1)
                .expect("Already checked length of *nums* is even, and chunk size is 2");

            std::iter::repeat_n(
                *val,
                usize::try_from(*freq).expect("No overflow since *freq* is at most 100"),
            )
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

        assert_eq!(decompress(&nums), Ok(vec![2, 4, 4, 4]));
    }

    #[test]
    fn test_2() {
        let nums = [1, 1, 2, 3];

        assert_eq!(decompress(&nums), Ok(vec![1, 3, 3]));
    }

    #[test]
    fn test_3() {
        let nums = [1, 2, 3];

        assert!(decompress(&nums).is_err());
    }
}
