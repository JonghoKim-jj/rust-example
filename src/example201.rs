fn double_slice_i32(input: &[i32]) -> Vec<i32> {
    let n = input.len();
    let mut result = Vec::with_capacity(2 * n);

    // Append the input vector twice
    result.extend_from_slice(&input);
    result.extend_from_slice(&input);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![5, 4, 3, 2, 1];
        let doubled = double_slice_i32(&a);
        assert_eq!(doubled, vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1]);
    }
}
