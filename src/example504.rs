use std::collections::HashMap;

#[allow(clippy::map_entry)]
fn decode(key: &str, message: &str) -> Result<String, &'static str> {
    if !(26..=2_000).contains(&key.len()) {
        return Err("Length of *key* must be between 26 and 2,000");
    }

    if !('a'..='z').all(|char| key.contains(char)) {
        return Err("Every alphabet a-z must be included in the *key* at least once");
    }

    let mut replace_table: HashMap<char, char> = HashMap::new();
    let mut remaining_rt_value = 'a'..='z'; // 'rt' stands for 'replace table'

    key.chars().filter(char::is_ascii_lowercase).for_each(|k| {
        if !replace_table.contains_key(&k) {
            replace_table.insert(
                k,
                remaining_rt_value
                    .next()
                    .expect("The iter is non-empty and checked in each iteration"),
            );
        }
    });

    if !remaining_rt_value.is_empty() {
        return Err("Not all key is mapped. Check your key again");
    }

    let decoded_message = message
        .chars()
        .map(|key_or_whitespace| {
            *replace_table
                .get(&key_or_whitespace)
                .unwrap_or(&key_or_whitespace)
        })
        .collect();

    Ok(decoded_message)
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn test_1() {
        let key = "the quick brown fox jumps over the lazy dog";
        let message = "vkbs bs t suepuv";
        let result = decode(key, message);

        assert_eq!(result, Ok("this is a secret".to_string()));
    }

    #[test]
    fn test_2() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo";
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb";
        let result = decode(key, message);

        assert_eq!(
            result,
            Ok("the five boxing wizards jump quickly".to_string())
        );
    }

    #[test]
    fn test_3() {
        let key: String = ('a'..='y').collect();
        let message = "will it work";
        let result = decode(&key, message);

        assert!(result.is_err());
    }

    #[test]
    fn test_4() {
        // *key* to be a string of repeating a~z 50 times
        let key: String = iter::repeat_n('a'..='z', 50).flatten().collect();
        let message = "so long key";
        let result = decode(&key, message);

        assert!(result.is_ok_and(|x| x == *"so long key"));
    }
}
