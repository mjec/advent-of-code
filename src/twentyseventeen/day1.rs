use return_codes;

make_dispatch_and_help!(
    "advent-of-code 2017 1",
    "1" => part1 => part1_help => "<digit-string>" => "Part 1: sum of digit equal to the next digit",
    "2" => part2 => part2_help => "<digit-string>" => "Part 2: sum of digits half way across from themselves"
);

fn part1(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 1, part1_help);

    if args[0].is_empty() {
        return Err(return_codes::BAD_DATA);
    }

    println!("{}", sum_of_consecutive_equal_digits(&args[0]));
    Ok(())
}

fn part2(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 1, part2_help);

    args[0].retain(|c| c.is_numeric());

    if args[0].is_empty() || args[0].len() % 2 == 1 {
        return Err(return_codes::BAD_DATA);
    }

    println!("{}", sum_of_oppsite_equal_digits(&args[0]));
    Ok(())
}

fn sum_of_consecutive_equal_digits(s: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut characters = s.chars().filter(|c| c.is_numeric()).peekable();
    let first_character = *characters.peek().unwrap();
    loop {
        let this_digit = characters.next();
        if this_digit.is_none() {
            break;
        }
        let next_digit = characters.peek().unwrap_or(&first_character);
        sum += if this_digit.unwrap() == *next_digit {
            this_digit.unwrap().to_digit(10).unwrap()
        } else {
            0
        }
    }
    sum
}

fn sum_of_oppsite_equal_digits(s: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i < s.len() {
        let j = (i + (s.len() / 2)) % s.len();
        if s[i..=i] == s[j..=j] {
            sum += s[i..=i].chars().next().unwrap().to_digit(10).unwrap();
        }
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_consecutive_equal_digits() {
        assert_eq!(3, sum_of_consecutive_equal_digits(&String::from("1122")));
        assert_eq!(4, sum_of_consecutive_equal_digits(&String::from("1111")));
        assert_eq!(0, sum_of_consecutive_equal_digits(&String::from("1234")));
        assert_eq!(
            9,
            sum_of_consecutive_equal_digits(&String::from("91212129"))
        );
    }

    #[test]
    fn test_sum_of_oppsite_equal_digits() {
        assert_eq!(6, sum_of_oppsite_equal_digits(&String::from("1212")));
        assert_eq!(0, sum_of_oppsite_equal_digits(&String::from("1221")));
        assert_eq!(4, sum_of_oppsite_equal_digits(&String::from("123425")));
        assert_eq!(12, sum_of_oppsite_equal_digits(&String::from("123123")));
        assert_eq!(4, sum_of_oppsite_equal_digits(&String::from("12131415")));
    }
}
