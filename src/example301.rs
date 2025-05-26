fn check_assumption(input: i32) -> bool {
    (1000..=9999).contains(&input)
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
        remaining /= 10;
    }

    // smallest digit first (ex) 0 0 4 9
    // If you are curious why sort() wasn't used,
    // see: https://rust-lang.github.io/rust-clippy/master/index.html#stable_sort_primitive
    digits.sort_unstable();

    digits
}

// Reference: https://users.rust-lang.org/t/why-are-not-min-and-max-macros-in-the-std/9730/2
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+ $(,)?) => (::std::cmp::min($x, min!($($z),*)));
}

fn solution1(input: i32) -> Result<i32, &'static str> {
    if !check_assumption(input) {
        return Err("Input must be four digits");
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

fn solution2(input: i32) -> Result<i32, &'static str> {
    if !check_assumption(input) {
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
    fn test_macro_1() {
        assert_eq!(1, min!(1));
    }

    #[test]
    fn test_macro_2() {
        // Works well with trailing comma
        assert_eq!(3, min!(3, 4, 5,));
    }

    #[test]
    fn test_macro_3() {
        // You can also use [] for macro input
        assert_eq!(3, min![3, 4, 5,]);
    }

    #[test]
    fn test_macro_4() {
        // You can also use {} for macro input
        assert_eq!(3, min! {3, 4, 5,});
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
        assert!(solution1(999).is_err());
    }

    #[test]
    fn test_fail_2() {
        assert!(solution1(10_000).is_err());
    }
}
