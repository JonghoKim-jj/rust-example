use std::collections::{BTreeMap, btree_map};

fn decode(key: &str, message: &str) -> Result<String, &'static str> {
    if !(26..=2_000).contains(&key.len()) {
        return Err("Length of *key* must be between 26 and 2,000");
    }

    if !('a'..='z').all(|char| key.contains(char)) {
        return Err("Every alphabet a-z must be included in the *key* at least once");
    }

    let mut replace_table: BTreeMap<char, char> = BTreeMap::new();
    let mut remaining_rt_value = 'a'..='z'; // 'rt' stands for 'replace table'

    for k in key.chars() {
        if k.is_ascii_lowercase() {
            if let btree_map::Entry::Vacant(entry) = replace_table.entry(k) {
                if let Some(v) = remaining_rt_value.next() {
                    entry.insert(v);
                } else {
                    break;
                }
            }
        }
    }

    if !remaining_rt_value.is_empty() {
        return Err("Not all key is mapped. Check your key again");
    }

    let decoded_message = message
        .chars()
        .map(|k| match k {
            'a'..='z' => replace_table
                .get(&k)
                .expect("Checked all keys are mapped")
                .to_owned(),
            other_char => other_char,
        })
        .collect();

    Ok(decoded_message)
}

#[cfg(test)]
mod tests {
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
}
