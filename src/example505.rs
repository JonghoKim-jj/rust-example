use std::collections::LinkedList;

// Reference: https://rust-lang.github.io/rust-clippy/master/index.html#linkedlist
#[allow(clippy::linkedlist)]
fn convert_list_to_integer(list: &LinkedList<i32>) -> Result<u32, &'static str> {
    if list.is_empty() {
        return Err("The linked list must be non-empty");
    }

    if list.len() > 30 {
        return Err("Length of the linked list should not exceed 30");
    }

    if !list.iter().all(|&item| item == 0 || item == 1) {
        return Err("*item* must be 0 or 1");
    }

    let integer = list
        .iter()
        .fold(0, |acc, item| (acc << 1) | item)
        .try_into()
        .expect("Length of the linked list will not exceed 30, so the result will not exceed 2^30, no overflow");

    Ok(integer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let list = LinkedList::from([1, 0, 1]);
        let result = convert_list_to_integer(&list);

        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_2() {
        let list = LinkedList::from([0]);
        let result = convert_list_to_integer(&list);

        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_3() {
        let list = LinkedList::from([]);
        let result = convert_list_to_integer(&list);

        assert!(result.is_err());
    }

    #[test]
    fn test_4() {
        let list = LinkedList::from([1; 31]);
        let result = convert_list_to_integer(&list);

        assert!(result.is_err());
    }
}
