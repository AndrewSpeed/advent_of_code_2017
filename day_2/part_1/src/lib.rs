pub fn calculate_checksum(input: &str) -> i32 {
    let split_string: Vec<&str> = input.split('\n').collect();

    let rows = convert_rows(split_string);
    let row_differences: Vec<i32> = rows.iter().map(|row| calculate_difference(row)).collect();

    return row_differences.iter().fold(
        0,
        |acc, &difference| acc + difference,
    );
}

fn convert_rows(rows: Vec<&str>) -> Vec<Vec<i32>> {
    return rows.iter().map(|row| convert_row(row)).collect();
}

fn convert_row(row: &str) -> Vec<i32> {
    return row.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
}

fn calculate_difference(values: &Vec<i32>) -> i32 {
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
        assert_eq!(calculate_difference(&vec![5, 1, 9, 5]), 8);
    }

    #[test]
    fn calculate_difference_can_find_difference_with_largest_before_smallest() {
        assert_eq!(calculate_difference(&vec![7, 5, 3]), 4);
    }

    #[test]
    fn calculate_difference_can_find_difference_with_largest_after_smallest() {
        assert_eq!(calculate_difference(&vec![2, 4, 6, 8]), 6);
    }

    #[test]
    fn convert_rows_converts_spreadsheet_correctly() {
        assert_eq!(
            convert_rows(vec!["5 1 9 5", "7 5 3", "4 6 8"]),
            vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![4, 6, 8]]
        );
    }

    #[test]
    fn convert_row_converts_a_row_correctly() {
        assert_eq!(convert_row("5 1 9 5"), vec![5, 1, 9, 5]);
    }

    #[test]
    fn calculate_checksum_works_correctly() {
        let input = "5 1 9 5\n7 5 3\n2 4 6 8";
        assert_eq!(calculate_checksum(input), 18);
    }
}
