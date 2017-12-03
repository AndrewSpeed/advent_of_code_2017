fn solve_captcha(captcha: &str) -> i64 {
    let captcha_numbers: Vec<i64> = String::from(captcha).chars().map(|c| i64::from(c.to_digit(10).unwrap())).collect();

    let mut result = 0;

    for (index, number) in captcha_numbers.iter().enumerate() {
        if matches_next_number(captcha_numbers.as_slice(), index) {
            result += number;
        }
    }

    return result;
}

fn matches_next_number(numbers: &[i64], index: usize) -> bool {
    let next_index;

    if index == (numbers.len() - 1) {
        next_index = 0;
    } else {
        next_index = index + 1;
    }

    return numbers[index] == numbers[next_index];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_a_captcha_with_two_repeated_numbers() {
        assert_eq!(3, solve_captcha("1122"));
    }

    #[test]
    fn it_solves_a_captcha_with_one_number_repeated_multiple_times() {
        assert_eq!(4, solve_captcha("1111"));
    }

    #[test]
    fn it_solves_a_captcha_with_no_repeated_numbers() {
        assert_eq!(0, solve_captcha("1234"));
    }

    #[test]
    fn it_solves_a_captcha_with_the_last_number_matchins_the_first() {
        assert_eq!(9, solve_captcha("91212129"));
    }

    #[test]
    fn matches_next_number_returns_true_if_next_index_value_is_equal() {
        assert_eq!(true, matches_next_number(vec![1, 1].as_slice(), 0));
    }

    #[test]
    fn matches_next_number_returns_false_if_next_index_value_is_not_equal() {
        assert_eq!(false, matches_next_number(vec![1, 2].as_slice(), 0));
    }

    #[test]
    fn matches_next_number_returns_true_if_last_index_is_compared_to_first() {
        assert_eq!(true, matches_next_number(vec![1, 2, 1].as_slice(), 2));
    }

    #[test]
    fn matches_next_number_returns_false_if_last_index_value_does_not_match_first_index_value() {
        assert_eq!(false, matches_next_number(vec![1, 2, 3].as_slice(), 2));
    }
}
