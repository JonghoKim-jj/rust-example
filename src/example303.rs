fn greatest_candy(candies: &[u32], extra_candies: u32) -> Result<Vec<bool>, &str> {
    if !(2_usize..=100).contains(&candies.len()) {
        return Err("Number of kids must be between 2 and 100");
    }

    if !(1..=50).contains(&extra_candies) {
        return Err("Extra candies must be between 1 and 50");
    }

    if !candies.iter().all(|item| (1..=100).contains(item)) {
        return Err("Number of candies a kid have must be between 1 and 100");
    }

    let max_candies = candies
        .iter()
        .max()
        .expect("Already checked the number of kids are at least 2, so maximum number of candies must exist");

    let result = candies
        .iter()
        .map(|item| *max_candies <= item + extra_candies)
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        assert_eq!(
            greatest_candy(&candies, extra_candies),
            Ok(vec![true, true, true, false, true])
        );
    }

    #[test]
    fn test_2() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        assert_eq!(
            greatest_candy(&candies, extra_candies),
            Ok(vec![true, false, false, false, false])
        );
    }

    #[test]
    fn test_3() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        assert_eq!(
            greatest_candy(&candies, extra_candies),
            Ok(vec![true, false, true])
        );
    }
}
