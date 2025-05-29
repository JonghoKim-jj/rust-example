use std::collections::HashMap;

// Number of Good Pairs: https://leetcode.com/problems/number-of-good-pairs/description/
fn combination(n: u32) -> u32 {
    n * (n - 1) / 2
}

fn num_good_pairs(nums: &[u32]) -> Result<u32, &'static str> {
    if !(1..=100).contains(&nums.len()) {
        return Err("Input length must be between 1 and 100");
    }

    if !nums.iter().all(|item| (1..=100).contains(item)) {
        return Err("All numbers must be between 1 and 100");
    }

    // key: item, value: count
    let mut item_counts: HashMap<u32, u32> = HashMap::new();
    for &num in nums {
        item_counts
            .entry(num)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    let result = item_counts
        .values()
        .fold(0_u32, |acc, &item| acc + combination(item));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(num_good_pairs(&nums).unwrap(), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(num_good_pairs(&nums).unwrap(), 6);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3];
        assert_eq!(num_good_pairs(&nums).unwrap(), 0);
    }
}
