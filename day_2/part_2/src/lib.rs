use std::ops::Rem;

pub fn sum_divisible_values(input: &str) -> i32 {
    let split_string: Vec<&str> = input.split('\n').collect();

    let rows = convert_rows(split_string);
    let row_division_values: Vec<i32> = rows.iter().map(|row| calculate_division(row)).collect();

    return row_division_values.iter().fold(0, |acc, &difference| {
        acc + difference
    });
}

fn convert_rows(rows: Vec<&str>) -> Vec<Vec<i32>> {
    return rows.iter().map(|row| convert_row(row)).collect();
}

fn convert_row(row: &str) -> Vec<i32> {
    return row.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
}

fn calculate_division(values: &Vec<i32>) -> i32 {
    // values.iter().enumerate().map(|index, value| {});

    return 0;
}

fn get_result_of_evenly_divisible_values(values: &Vec<i32>) -> i32 {
    let results = values.iter().enumerate().map(|(index, value)| {
        let greater_values = values.split_at(index + 1).2;
        return get_divisible_values_for(value, greater_values);
    });

    match results {
        Some((divisor, value)) => return value / divisor,
        None => {}
    }

    panic!("No evenly divisible values found in {:?}", values);
}

fn get_divisible_values_for<'a, 'b>(
    divisor: &'a i32,
    values: &'b Vec<i32>,
) -> Option<(&'a i32, &'b i32)> {
    let results: Vec<&i32> = values
        .iter()
        .filter(|value| { return value.rem(divisor) == 0; })
        .collect();

    if results.is_empty() {
        return None;
    } else {
        return Some((divisor, results.first().unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_divisible_values_for_works_if_values_contains_divisible_value() {
        assert_eq!(get_divisible_values_for(&3, &vec![4, 5, 6]), Some((&3, &6)));
    }

    #[test]
    fn get_divisible_values_for_works_if_values_does_not_contain_divisible_value() {
        assert_eq!(get_divisible_values_for(&10, &vec![1, 2, 3]), None);
    }

    #[test]
    fn get_result_of_evenly_divisible_values_works_correctly() {
        assert_eq!(get_result_of_evenly_divisible_values(&vec![2, 3, 5, 6]), 3);
    }

    #[test]
    fn calculate_division_with_lower_value_before_greater() {
        assert_eq!(calculate_division(&vec![5, 9, 2, 8]), 4);
    }

    #[test]
    fn calculate_division_with_greater_value_before_lower() {
        assert_eq!(calculate_division(&vec![9, 4, 7, 3]), 3);
    }

    #[test]
    fn calculate_division_with_value_between_lower_and_greater() {
        assert_eq!(calculate_division(&vec![3, 8, 6, 5]), 2);
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
        let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(sum_divisible_values(input), 9);
    }
}
