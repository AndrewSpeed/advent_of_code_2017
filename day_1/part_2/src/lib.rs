pub fn solve_captcha(captcha: String) -> u32 {
    println!("Solving: {}", captcha);
    let bytes = captcha.into_bytes();
    println!("Bytes: {:?}", bytes);

    let mut result = 0;

    for (index, byte) in bytes.iter().enumerate() {
        if matches_next_number(bytes.as_slice(), index) {
            let number = char::from(*byte).to_digit(16).unwrap();
            println!("Number: {}", number);
            result += number;
            println!("Updated result to: {}", result);
        }
    }

    return result;
}

fn matches_next_number(numbers: &[u8], index: usize) -> bool {
    let comparison_index = get_comparison_index(numbers, index);
    println!("Comparing index {} to index {} in {:?}", index, comparison_index, numbers);

    return numbers[index] == numbers[comparison_index];
}

fn get_comparison_index(array: &[u8], index: usize) -> usize {
    let array_length = array.len();
    return (index + (array_length / 2)) % array_length;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_a_captcha_with_two_repeated_numbers() {
        assert_eq!(6, solve_captcha(String::from("1212")));
    }

    #[test]
    fn it_solves_a_captcha_with_one_number_repeated_multiple_times() {
        assert_eq!(0, solve_captcha(String::from("1221")));
    }

    #[test]
    fn it_solves_a_captcha_with_no_repeated_numbers() {
        assert_eq!(4, solve_captcha(String::from("123425")));
    }

    #[test]
    fn it_solves_a_captcha_with_the_last_number_matching_the_first() {
        assert_eq!(12, solve_captcha(String::from("123123")));
    }

    #[test]
    fn it_solves_a_captcha() {
        assert_eq!(4, solve_captcha(String::from("12131415")));
    }

    #[test]
    fn matches_next_number_returns_true_if_halfway_digit_matches() {
        assert_eq!(true, matches_next_number(vec![1, 2, 1, 2].as_slice(), 0));
    }

    #[test]
    fn matches_next_number_returns_true_if_halfway_digit_is_before_it() {
        assert_eq!(true, matches_next_number(vec![1, 2, 1, 2].as_slice(), 2));
    }

    #[test]
    fn matches_next_number_returns_false_if_halfway_digit_does_not_match() {
        assert_eq!(false, matches_next_number(vec![1, 2, 3, 1].as_slice(), 1));
    }

    #[test]
    fn get_comparison_index_for_first_element() {
        assert_eq!(2, get_comparison_index(vec![1, 2, 3, 4].as_slice(), 0));
    }

    #[test]
    fn get_comparison_index_for_second_element() {
        assert_eq!(3, get_comparison_index(vec![1, 2, 3, 4].as_slice(), 1));
    }

    #[test]
    fn get_comparison_index_for_third_element() {
        assert_eq!(0, get_comparison_index(vec![1, 2, 3, 4].as_slice(), 2));
    }

    #[test]
    fn get_comparison_index_for_final_element() {
        assert_eq!(1, get_comparison_index(vec![1, 2, 3, 4].as_slice(), 3));
    }
}
