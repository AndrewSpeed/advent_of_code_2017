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
        assert_eq!(3, solve_captcha(String::from("1122")));
    }

    #[test]
    fn it_solves_a_captcha_with_one_number_repeated_multiple_times() {
        assert_eq!(4, solve_captcha(String::from("1111")));
    }

    #[test]
    fn it_solves_a_captcha_with_no_repeated_numbers() {
        assert_eq!(0, solve_captcha(String::from("1234")));
    }

    #[test]
    fn it_solves_a_captcha_with_the_last_number_matchins_the_first() {
        assert_eq!(9, solve_captcha(String::from("91212129")));
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
