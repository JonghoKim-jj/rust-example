fn shuffle(s: &str, indices: &[usize]) -> Result<String, &'static str> {
    if !((1..=100).contains(&s.len()) && (1..=100).contains(&indices.len())) {
        return Err("Length of *s* and *indices* must be between 1 and 100");
    }

    if s.len() != indices.len() {
        return Err("Length of *s* and *indices* must be equal");
    }

    // Checking Assumption
    //  - This assumption should be checked for shuffling/permutation
    //  - Defining length of *indices* as *n*,
    //    check all values of *indices* are unique and in the range 0..n
    //  - Success of each iteration gives that idx is included in *indices*,
    //    while idx is in the range 0, 1, ..., n-1.
    if !(0..indices.len()).all(|idx| indices.contains(&idx)) {
        return Err("All indices must be unique and in the range 0..n");
    }

    let mut buff: Vec<_> = s.chars().zip(indices.iter()).collect();
    buff.sort_unstable_by_key(|(_, idx)| **idx);

    let shuffled = buff.iter().map(|(char, _)| char).collect();

    Ok(shuffled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "codeleet";
        let indices = [4, 5, 6, 7, 0, 2, 1, 3];
        let result = shuffle(s, &indices);

        assert_eq!(result, Ok("leetcode".to_string()));
    }

    #[test]
    fn test_2() {
        let s = "abc";
        let indices = [0, 1, 2];
        let result = shuffle(s, &indices);

        assert_eq!(result, Ok("abc".to_string()));
    }

    #[test]
    fn test_3() {
        let s = "errorcheck";
        let indices = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = shuffle(s, &indices);

        assert!(result.is_err());
    }
}
