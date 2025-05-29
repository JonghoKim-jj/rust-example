use std::collections::BTreeMap;

fn solution(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    if !(2..=500).contains(&nums.len()) {
        return Err("Length of *num* must between 2 and 500");
    }

    let mut item_count = BTreeMap::<i32, i32>::new();

    for &item in nums {
        if !(0..=100).contains(&item) {
            return Err("Each num[i] must be between 0 and 100");
        }
        item_count
            .entry(item)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    let sum_smaller_num_counts = [0]
        .iter()
        .chain(item_count.values())
        .take(item_count.len())
        .scan(0, |sum, value| {
            *sum += value;
            Some(*sum)
        });

    let dict: BTreeMap<&i32, i32> = item_count.keys().zip(sum_smaller_num_counts).collect();
    let result: Vec<i32> = nums
        .iter()
        .map(|item| dict.get(item).expect("").to_owned())
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
