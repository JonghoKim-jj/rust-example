fn number_of_steps(num: u32) -> Result<u32, &'static str> {
    if num > 1_000_000 {
        return Err("*num* must be between 0 and 1,000,000");
    }

    let mut curr = num;
    let mut step: u32 = 0;
    while curr != 0 {
        step += 1;
        curr = if curr % 2 == 0 { curr / 2 } else { curr - 1 };
    }

    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(number_of_steps(14), Ok(6));
    }

    #[test]
    fn test_2() {
        assert_eq!(number_of_steps(8), Ok(4));
    }

    #[test]
    fn test_3() {
        assert_eq!(number_of_steps(123), Ok(12));
    }

    #[test]
    fn test_4() {
        assert!(number_of_steps(0).is_ok());
        assert!(number_of_steps(1_000_000).is_ok());
        assert!(number_of_steps(1_000_001).is_err());
    }
}
