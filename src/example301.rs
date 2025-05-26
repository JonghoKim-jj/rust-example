#[derive(Debug)]
enum Example301Result {
    Ok(i32),
    Err(Example301Error),
}

#[derive(Debug, Clone, PartialEq)]
enum Example301Error {
    NotFourDigits,
}

use Example301Error::*;
use Example301Result::{Err, Ok};

impl PartialEq for Example301Result {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => a == b,
            (Err(c), Err(d)) => c == d,
            _ => false,
        }
    }
}

fn check_assumption(input: i32) -> bool {
    if 1000 <= input && input <= 9999 {
        true
    } else {
        false
    }
}

fn digits_to_num(input: &[i32]) -> i32 {
    input.iter().fold(0, |acc, digit| 10 * acc + digit)
}

fn num_to_digits(input: i32) -> Vec<i32> {
    let mut remaining = input;
    let mut digits: Vec<i32> = Vec::new();

    while remaining > 0 {
        let digit = remaining % 10;
        digits.push(digit);
        remaining = remaining / 10;
    }

    // smallest digit first (ex) 0 0 4 9
    digits.sort();

    digits
}

// Reference: https://users.rust-lang.org/t/why-are-not-min-and-max-macros-in-the-std/9730/2
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+ $(,)?) => (::std::cmp::min($x, min!($($z),*)));
}

fn solution1(input: i32) -> Example301Result {
    if check_assumption(input) == false {
        return Err(NotFourDigits);
    }

    let digits = num_to_digits(input);

    // Brute-force Answer
    let answer: i32 = min!(
        digits_to_num(&[digits[0]]) + digits_to_num(&[digits[1], digits[2], digits[3]]),
        digits_to_num(&[digits[1]]) + digits_to_num(&[digits[0], digits[2], digits[3]]),
        digits_to_num(&[digits[2]]) + digits_to_num(&[digits[0], digits[1], digits[3]]),
        digits_to_num(&[digits[3]]) + digits_to_num(&[digits[0], digits[1], digits[2]]),
        digits_to_num(&[digits[0], digits[1]]) + digits_to_num(&[digits[2], digits[3]]),
        digits_to_num(&[digits[0], digits[2]]) + digits_to_num(&[digits[1], digits[3]]),
        digits_to_num(&[digits[0], digits[3]]) + digits_to_num(&[digits[1], digits[2]]),
    );

    Ok(answer)
}

fn solution2(input: i32) -> Example301Result {
    if check_assumption(input) == false {
        return Err(NotFourDigits);
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
    fn test_macro_1() {
        assert_eq!(1, min!(1))
    }

    #[test]
    fn test_macro_2() {
        // Works well with trailing comma
        assert_eq!(3, min!(3, 4, 5,))
    }

    #[test]
    fn test_macro_3() {
        // You can also use [] for macro input
        assert_eq!(3, min![3, 4, 5,])
    }

    #[test]
    fn test_macro_4() {
        // You can also use {} for macro input
        assert_eq!(3, min! {3, 4, 5,})
    }

    #[test]
    fn test_1() {
        assert_eq!(solution1(2932), Ok(52));
        assert_eq!(solution2(2932), Ok(52));
    }

    #[test]
    fn test_2() {
        assert_eq!(solution1(4009), Ok(13));
        assert_eq!(solution2(4009), Ok(13));
    }

    #[test]
    fn test_3() {
        assert_eq!(solution1(1234), Ok(37));
        assert_eq!(solution2(1234), Ok(37));
    }

    #[test]
    fn test_4() {
        assert_eq!(solution1(9876), Ok(147));
        assert_eq!(solution2(9876), Ok(147));
    }

    #[test]
    fn test_5() {
        assert_eq!(solution1(8004), Ok(12));
        assert_eq!(solution2(8004), Ok(12));
    }

    #[test]
    fn test_fail_1() {
        assert_eq!(solution1(999), Err(NotFourDigits))
    }

    #[test]
    fn test_fail_2() {
        assert_eq!(solution1(10_000), Err(NotFourDigits));
    }
}
