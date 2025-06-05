fn num_to_digits(input: i32) -> Vec<i32> {
    let mut remaining = input;
    let mut digits: Vec<i32> = Vec::new();

    while remaining > 0 {
        let digit = remaining % 10;
        digits.push(digit);
        remaining /= 10;
    }

    // smallest digit first (ex) 0 0 4 9
    // If you are curious why sort() wasn't used,
    // see: https://rust-lang.github.io/rust-clippy/master/index.html#stable_sort_primitive
    digits.sort_unstable();

    digits
}

fn min_sum_four_digits(input: i32) -> Result<i32, &'static str> {
    if !(1000..=9999).contains(&input) {
        return Err("Input must be four digits");
    }

    let digits = num_to_digits(input);

    // Simple Answer
    let answer = 10_i32 * (digits[0] + digits[1]) + digits[2] + digits[3];

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_sum_four_digits(2932), Ok(52));
    }

    #[test]
    fn test_2() {
        assert_eq!(min_sum_four_digits(4009), Ok(13));
    }

    #[test]
    fn test_3() {
        assert_eq!(min_sum_four_digits(1234), Ok(37));
    }

    #[test]
    fn test_4() {
        assert_eq!(min_sum_four_digits(9876), Ok(147));
    }

    #[test]
    fn test_5() {
        assert_eq!(min_sum_four_digits(8004), Ok(12));
    }

    #[test]
    fn test_fail_1() {
        assert!(min_sum_four_digits(999).is_err());
    }

    #[test]
    fn test_fail_2() {
        assert!(min_sum_four_digits(10_000).is_err());
    }
}
