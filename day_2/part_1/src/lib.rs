fn calculate_checksum(input: &str) -> i32 {
    return 0;
}

fn calculate_difference(values: Vec<i32>) -> i32 {
    let mut sorted_values = values.clone();
    sorted_values.sort();
    let smalled_value = sorted_values.first().unwrap();
    let largest_value = sorted_values.last().unwrap();

    return largest_value - smalled_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_difference_can_find_difference_with_largest_beside_smallest() {
        assert_eq!(calculate_difference(vec![5, 1, 9, 5]), 8);
    }

    #[test]
    fn calculate_difference_can_find_difference_with_largest_before_smallest() {
        assert_eq!(calculate_difference(vec![7, 5, 3]), 4);
    }

    #[test]
    fn calculate_difference_can_find_difference_with_largest_after_smallest() {
        assert_eq!(calculate_difference(vec![2, 4, 6, 8]), 6);
    }

    #[test]
    fn calculate_checksum_works_correctly() {
        let input = "5 1 9 5\n7 5 3\n2 4 6 8";
        assert_eq!(calculate_checksum(input), 18);
    }
}
