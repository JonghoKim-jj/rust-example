fn richest_wealth(accounts: &[&[u32]]) -> Result<u32, &'static str> {
    if !(1_usize..=50).contains(&accounts.len()) {
        return Err("Number of customers must be between 1 and 50.");
    }

    if !accounts
        .iter()
        .all(|&customer_accounts| (1_usize..=50).contains(&customer_accounts.len()))
    {
        return Err("Number of banks must be between 1 and 50.");
    }

    if !accounts.iter().all(|customer_accounts| {
        customer_accounts
            .iter()
            .all(|money| (1_u32..=100).contains(money))
    }) {
        return Err("Amounts of money in each account must be between 1 and 100.");
    }

    let richest_wealth = accounts
        .iter()
        .map(|&item| item.iter().sum())
        .max()
        .expect("Already checked there is at least 1 customer, thus the max exists");

    Ok(richest_wealth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let accounts: Vec<&[u32]> = vec![&[1_u32, 2, 3], &[3, 2, 1]];
        let result = richest_wealth(&accounts);
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn test_2() {
        let accounts: Vec<&[u32]> = vec![&[1_u32, 5], &[7, 3], &[3, 5]];
        let result = richest_wealth(&accounts);
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_3() {
        let accounts: Vec<&[u32]> = vec![&[2_u32, 8, 7], &[7, 1, 3], &[1, 9, 5]];
        let result = richest_wealth(&accounts);
        assert_eq!(result, Ok(17));
    }
}
