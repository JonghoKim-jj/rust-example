enum RuleKey {
    Type = 0,
    Color = 1,
    Name = 2,
}

impl TryFrom<&str> for RuleKey {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "type" => Ok(RuleKey::Type),
            "color" => Ok(RuleKey::Color),
            "name" => Ok(RuleKey::Name),
            _ => Err("Rule key must be among 'type', 'color', 'name'"),
        }
    }
}

impl From<&RuleKey> for usize {
    fn from(value: &RuleKey) -> Self {
        match value {
            RuleKey::Type => 0,
            RuleKey::Color => 1,
            RuleKey::Name => 2,
        }
    }
}

type Item<'a> = [&'a str; 3];

fn count_matching_items(
    items: &[&Item],
    rule_key: &RuleKey,
    rule_value: &str,
) -> Result<u32, &'static str> {
    if !(1..=10_000).contains(&items.len()) {
        return Err("Length of *items* must be between 1 and 10,000");
    }

    let count: u32 = items
        .iter()
        .filter(|&item| {
            *item
                .get(usize::from(rule_key))
                .expect("index using rule_key does not exceed range")
                == rule_value
        })
        .count()
        .try_into()
        .expect("Length of *items* will not exceed 10,000, thus no overflow");

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let items = [
            &["phone", "blue", "pixel"],
            &["computer", "silver", "lenovo"],
            &["phone", "gold", "iphone"],
        ];

        let rule_key: RuleKey = "color".try_into().expect("Example input is correct");
        let rule_value = "silver";

        let result = count_matching_items(&items, &rule_key, rule_value);

        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_2() {
        let items = [
            &["phone", "blue", "pixel"],
            &["computer", "silver", "phone"],
            &["phone", "gold", "iphone"],
        ];
        let rule_key: RuleKey = "type".try_into().expect("Example input is correct");
        let rule_value = "phone";

        let result = count_matching_items(&items, &rule_key, rule_value);

        assert_eq!(result, Ok(2));
    }
}
