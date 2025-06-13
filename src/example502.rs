use std::iter::repeat_n;

fn decompress(nums: &[u32]) -> Result<Vec<u32>, &'static str> {
    if !(2..=100).contains(&nums.len()) {
        return Err("Length of *nums* must be between 2 and 100");
    }

    if nums.len() % 2 == 1 {
        return Err("Length of *nums* must be even");
    }

    if !nums.chunks_exact(2).all(|item| {
        let freq = item.first().expect("Chunk has at least 1 element");
        (1..=100).contains(freq)
    }) {
        return Err("*freq* must be between 1 and 100");
    }

    let result: Vec<u32> = nums
        .chunks_exact(2)
        .flat_map(|item| {
            let [freq, val] = item
                .try_into()
                .expect("Always slice of 2 items due to chunks_exact()");

            repeat_n(
                val,
                usize::try_from(freq).expect("No overflow since *freq* is at most 100"),
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
