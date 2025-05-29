fn solution(jewels: &str, stones: &str) -> Result<u32, &'static str> {
    if !((1..=50).contains(&jewels.len()) && (1..=50).contains(&stones.len())) {
        return Err("Length of jewels and stones must be between 1 and 50");
    }

    if !(jewels.chars().all(|char| char.is_ascii_alphabetic())
        && stones.chars().all(|char| char.is_ascii_alphabetic()))
    {
        return Err("All letter in jewels and stones must be English alphabet");
    }

    let count = stones.chars().fold(
        0_u32,
        |sum, char| {
            if jewels.contains(char) { sum + 1 } else { sum }
        },
    );

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
