fn solution(n: u32) -> Result<u32, &'static str> {
    if !(1..=100_000).contains(&n) {
        return Err("The *n* must be between 1 and 10,000");
    }

    let mut digits: Vec<u32> = Vec::new();
    let mut curr = n;

    while curr > 0 {
        digits.push(curr % 10);
        curr /= 10;
    }

    let (sum, product) = digits
        .iter()
        .fold((0, 1), |acc, item| (acc.0 + item, acc.1 * item));

    Ok(sum.abs_diff(product))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 234;
        assert_eq!(solution(n), Ok(15));
    }

    #[test]
    fn test_2() {
        let n = 4421;
        assert_eq!(solution(n), Ok(21));
    }
}
