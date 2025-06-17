fn solution(jewels: &str, stones: &str) -> Result<u32, &'static str> {
    if !((1..=50).contains(&jewels.len()) && (1..=50).contains(&stones.len())) {
        return Err("Length of jewels and stones must be between 1 and 50");
    }

    if !(jewels.chars().all(|char| char.is_ascii_alphabetic())
        && stones.chars().all(|char| char.is_ascii_alphabetic()))
    {
        return Err("All letter in jewels and stones must be English alphabet");
    }

    let count: u32 = stones
        .chars()
        .filter(|&item| jewels.contains(item))
        .count()
        .try_into()
        .expect("The count can not exceed 50 because maximum length of jewels and stones must between 1 and 50");

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let jewels = "aA";
        let stones = "aAAbbbb";

        assert_eq!(solution(jewels, stones), Ok(3));
    }

    #[test]
    fn test_2() {
        let jewels = "z";
        let stones = "ZZ";

        assert_eq!(solution(jewels, stones), Ok(0));
    }
}
