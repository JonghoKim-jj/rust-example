fn defang(address: &str) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let address = "1.1.1.1";
        assert_eq!(defang(address), "1[.]1[.]1[.]1");
    }

    #[test]
    fn test_2() {
        let address = "255.100.50.0";
        assert_eq!(defang(address), "255[.]100[.]50[.]0");
    }
}
