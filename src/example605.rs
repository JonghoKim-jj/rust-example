#[allow(clippy::needless_for_each)]
fn min_moves_to_seat(seats: &[u32], students: &[u32]) -> Result<u32, &'static str> {
    if !(1..=100).contains(&seats.len()) {
        return Err("Number of *seats* must be between 1 and 100");
    }

    if !(1..=100).contains(&students.len()) {
        return Err("Number of *students* must be between 1 and 100");
    }

    if seats.len() != students.len() {
        return Err("Number of *seats* and *students* must be equal");
    }

    if !seats.iter().all(|item| *item <= 100) {
        return Err("All position of *seats* must be between 0 and 100");
    }

    if !students.iter().all(|item| *item <= 100) {
        return Err("All position of *students* must be between 0 and 100");
    }

    let mut remaining_seats = Vec::from(seats);
    remaining_seats.sort_unstable();

    let mut remaining_students = Vec::from(students);
    remaining_students.sort_unstable();

    Ok(remaining_seats
        .iter()
        .zip(remaining_students.iter())
        .map(|(&seat, &student)| seat.abs_diff(student))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let seats = [3, 1, 5];
        let students = [2, 7, 4];
        let result = min_moves_to_seat(&seats, &students);

        assert_eq!(result, Ok(4));
    }

    #[test]
    fn test_2() {
        let seats = [4, 1, 5, 9];
        let students = [1, 3, 2, 6];
        let result = min_moves_to_seat(&seats, &students);

        assert_eq!(result, Ok(7));
    }

    #[test]
    fn test_3() {
        let seats = [2, 2, 6, 6];
        let students = [1, 3, 2, 6];
        let result = min_moves_to_seat(&seats, &students);

        assert_eq!(result, Ok(4));
    }

    #[test]
    fn test_4() {
        let seats = [1, 2, 3];
        let students = [4, 5, 6, 7];

        assert!(min_moves_to_seat(&seats, &students).is_err());
    }

    #[test]
    fn test_5() {
        let seats1 = [0; 100];
        let students1 = [0; 100];
        let result1 = min_moves_to_seat(&seats1, &students1);

        let seats2 = [0; 101];
        let students2 = [0; 101];
        let result2 = min_moves_to_seat(&seats2, &students2);

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
}
