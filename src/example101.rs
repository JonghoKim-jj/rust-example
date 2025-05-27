fn add_two_integers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = 99;
        let b = 101;

        assert_eq!(add_two_integers(a, b), 200);
    }

    #[test]
    fn test2() {
        let c = 123;
        let d = 456;

        assert_eq!(add_two_integers(c, d), 579);
    }

    #[test]
    fn test3() {
        let e = 1;
        let f = 2;

        assert_ne!(add_two_integers(e, f), 4);
    }
}
