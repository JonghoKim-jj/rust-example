fn solution(sentences: &[&str]) -> Option<u32> {
    sentences
        .iter()
        .filter_map(|&sentence| u32::try_from(sentence.split_whitespace().count()).ok())
        .max()
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

        assert_eq!(solution(&sentences), Some(6));
    }

    #[test]
    fn test_2() {
        let sentences = ["please wait", "continue to fight", "continue to win"];

        assert_eq!(solution(&sentences), Some(3));
    }

    #[test]
    fn test_3() {
        use std::iter;
        let sentences: Vec<&str> = iter::repeat_n("Hello", 200).collect();

        assert!(solution(&sentences).is_some_and(|x| x == 1));
    }

    #[test]
    fn test_4() {
        let sentences: Vec<&str> = vec![];

        assert!(solution(&sentences).is_none());
    }
}
