fn solution(sentences: &[&str]) -> Result<u32, &'static str> {
    if !(1..=100).contains(&sentences.len()) {
        return Err("Number of sentences must be between 1 and 100");
    }

    let result: Result<u32, _> = sentences
        .iter()
        .map(|&sentence| sentence.split_whitespace().count())
        .max()
        .expect("At least one sentence exists, thus max exists")
        .try_into()
        .map_err(|_| "Error converting max_words usize -> u32");

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let sentences = [
            "alice and bob love leetcode",
            "i think so too",
            "this is great thanks very much",
        ];

        assert_eq!(solution(&sentences), Ok(6));
    }

    #[test]
    fn test_2() {
        let sentences = ["please wait", "continue to fight", "continue to win"];

        assert_eq!(solution(&sentences), Ok(3));
    }

    #[test]
    fn test_3() {
        use std::iter;
        let sentences: Vec<&str> = iter::repeat_n("Hello", 200).collect();

        assert!(solution(&sentences).is_err());
    }
}
