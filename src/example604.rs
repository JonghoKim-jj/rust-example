fn number_of_matches(n: u32) -> Result<u32, &'static str> {
    if !(1..=200).contains(&n) {
        return Err("*n* must be between i and 200");
    }

    // Each match reduces alive team by 1.
    // When a team is randomly picked and advance to next round, there is no match.
    // Therefore, if *n* teams in a tournament, (n-1) matches are needed
    Ok(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(number_of_matches(7), Ok(6));
    }

    #[test]
    fn test_2() {
        assert_eq!(number_of_matches(14), Ok(13));
    }

    #[test]
    fn test_3() {
        assert!(number_of_matches(200).is_ok());
        assert!(number_of_matches(201).is_err());
    }
}
