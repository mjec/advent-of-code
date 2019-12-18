use return_codes;
use std::io::{self, Read};

make_dispatch_and_help!(
    "advent-of-code 2017 2",
    "1" => part1 => part1_help => "" => "Part 1: sum of differences between min and max of each line (takes input on stdin)",
    "2" => part2 => part2_help => "" => "Part 2: sum of x / y for each line, where x | y for all distinct x, y in the line (takes input on stdin)"
);

fn part1(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 0, part1_help);

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
    }

    println!("{}", sum_of_difference_min_and_max_each_line(&buffer));
    Ok(())
}

fn part2(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 0, part2_help);

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
    }

    println!("{}", sum_of_possible_divisions_each_line(&buffer));
    Ok(())
}

fn sum_of_difference_min_and_max_each_line(s: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in s.lines() {
        let mut min: Option<i32> = None;
        let mut max: Option<i32> = None;
        for cell in line.split_whitespace() {
            if let Ok(n) = cell.parse::<i32>() {
                if min.is_none() || n < min.unwrap() {
                    min = Some(n);
                }
                if max.is_none() || n > max.unwrap() {
                    max = Some(n);
                }
            }
        }
        if min.is_some() && max.is_some() {
            sum += (max.unwrap() - min.unwrap()) as u32;
        }
    }

    sum
}

fn sum_of_possible_divisions_each_line(s: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in s.lines() {
        let mut set: Vec<i32> = Vec::new();
        for cell in line.split_whitespace() {
            if let Ok(n) = cell.parse::<i32>() {
                set.push(n);
            }
        }

        let mut i = 0;
        let mut j;
        while i < set.len() {
            j = i + 1;
            while j < set.len() {
                let (bigger, smaller) = if set[i] > set[j] {
                    (set[i], set[j])
                } else {
                    (set[j], set[i])
                };
                if bigger % smaller == 0 {
                    sum += bigger / smaller;
                }
                j += 1;
            }

            i += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_difference_min_and_max_each_line() {
        assert_eq!(
            18,
            sum_of_difference_min_and_max_each_line(&String::from(
                "5 1 9 5
7 5 3
2 4 6 8"
            ))
        );
    }

    #[test]
    fn test_sum_of_possible_divisions_each_line() {
        assert_eq!(
            9,
            sum_of_possible_divisions_each_line(&String::from(
                "5 9 2 8
9 4 7 3
3 8 6 5"
            ))
        );
    }

}
