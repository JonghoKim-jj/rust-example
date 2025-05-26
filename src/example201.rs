use std::{default, fmt::Display, vec};

fn double_vector_i32(input: Vec<i32>) -> Vec<i32> {
    let n = input.len();
    let mut result = Vec::with_capacity(2 * n);
    result.resize(2 * n, i32::default());

    for (idx, item) in input.into_iter().enumerate() {
        result[idx] = item;
        result[n + idx] = item;
    }

    result
}

fn double_vector<T: Default + Clone>(input: Vec<T>) -> Vec<T> {
    let n = input.len();
    let mut result = Vec::with_capacity(2 * n);
    result.resize(2 * n, T::default());

    for (idx, item) in input.into_iter().enumerate() {
        result[idx] = item.clone();
        result[n + idx] = item.clone();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_1() {
        let a = vec![5, 4, 3, 2, 1];
        let doubled = double_vector_i32(a);
        assert_eq!(doubled, vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_generic_1() {
        let a = vec!['a', 'b', 'c', 'd'];
        let doubled = double_vector(a);
        assert_eq!(doubled, vec!['a', 'b', 'c', 'd', 'a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_generic_2() {
        let a = vec![3.5, 12.0, 5.3];
        let doubled = double_vector(a);
        assert_eq!(doubled, vec![3.5, 12.0, 5.3, 3.5, 12.0, 5.3]);
    }
}
