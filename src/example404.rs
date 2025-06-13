fn sort_sentence(s: &str) -> Result<String, &'static str> {
    if !(2..=200).contains(&s.len()) {
        return Err("Length of *s* must be between 2 and 200");
    }

    if s.chars()
        .next()
        .expect("*s* must be non-empty string")
        .is_whitespace()
        || s.chars()
            .last()
            .expect("*s* must be non-empty string")
            .is_whitespace()
    {
        return Err("*s* contains no leading/trailing whitespace");
    }

    if !(1..=9).contains(&s.split_whitespace().count()) {
        return Err("Number of words *s* contains must be between 1 and 9");
    }

    // Item: (word, idx)
    let mut words_with_idx = s
        .split_whitespace()
        .map(|word_followed_by_idx| {
            let (word, idx_str) = word_followed_by_idx.split_at(word_followed_by_idx.len() - 1);
            idx_str
                .parse::<u32>()
                .map(|idx| (word, idx))
                .map_err(|_| "Error parsing index")
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Check all num among 1..=len is in the indices
    if !(1..=words_with_idx.len()).all(|num| {
        words_with_idx
            .iter()
            .any(|&(_, idx)| idx == num.try_into().expect("Length is up to 9, no overflow"))
    }) {
        return Err("Indexes must consist of 1..=len");
    }

    words_with_idx.sort_unstable_by_key(|&(_, idx)| idx);

    let sentence = words_with_idx
        .iter()
        .map(|&(word, _)| word)
        .collect::<Vec<_>>()
        .join(" ");

    Ok(sentence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "is2 sentence4 This1 a3";
        let sentence = String::from("This is a sentence");

        assert_eq!(sort_sentence(s), Ok(sentence));
    }

    #[test]
    fn test_2() {
        let s = "Myself2 Me1 I4 and3";
        let sentence = String::from("Me Myself and I");

        assert_eq!(sort_sentence(s), Ok(sentence));
    }

    #[test]
    fn test_3() {
        let s1 = "a9 b8 c7 d6 e5 f4 g3 h2 i1";
        let s2 = "a8 b7 c6 d5 e4 f3 g2 h1 i0";

        assert!(sort_sentence(s1).is_ok_and(|result| result == "i h g f e d c b a"));
        assert!(sort_sentence(s2).is_err());
    }

    #[test]
    fn test_4() {
        let s1 = "a4 b3 c2 d1";
        let s2 = "a5 b4 c3 d2";

        assert!(sort_sentence(s1).is_ok_and(|result| result == "d c b a"));
        assert!(sort_sentence(s2).is_err());
    }

    #[test]
    fn test_5() {
        let s = "   world2 Hello1   ";

        assert!(sort_sentence(s).is_err());
    }
}
