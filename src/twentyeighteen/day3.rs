use return_codes;
use std::collections::HashMap;
use std::io::{self, Read};

make_dispatch_and_help!(
    "advent-of-code 2018 3",
    "1" => part1 => part1_help => "" => "Part 1: number of square inches of cloth which are contested (takes line-delimited input on stdin)"
    // "2" => part2 => part2_help => "" => "Part 2: first number reached twice doing a cycle sum of inputs (takes line-delimited input on stdin)"
);

fn part1(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 0, part1_help);

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
    }

    println!("{}", count_of_contested_segments(&buffer));
    Ok(())
}

// fn part2(args: &mut Vec<String>) -> Result<(), i32> {
//     require_exactly_arguments!(args, 0, part1_help);

//     let mut buffer = String::new();
//     if let Err(e) = io::stdin().read_to_string(&mut buffer) {
//         println!("Unable to read from stdin: {:?}", e);
//         return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
//     }

//     println!("{}", first_reached_twice(&buffer));
//     Ok(())
// }

fn count_of_contested_segments(stdin: &str) -> u32 {
    let mut cloth: HashMap<(u32, u32), bool> = HashMap::new(); // False = 1 claim, True = 2+ claims
    let x_min = 0;
    let y_min = 0;
    let x_count = 0;
    let y_count = 0;
    let mut result: u32 = 0;

    for line in stdin.lines() {
        // Each line looks like this: "#1 @ 1,3: 4x4"
        for i in 0..x_count {
            for j in 0..y_count {
                let coord = ((x_min + i), (y_min + j));
                result += match cloth.get(&coord) {
                    Some(true) => 0,
                    Some(false) => {
                        cloth.insert(coord, true);
                        1
                    }
                    None => {
                        cloth.insert(coord, false);
                        0
                    }
                }
            }
        }
    }
    result
}

// fn first_reached_twice(stdin: &String) -> i32 {
//     let mut result: i32 = 0;
//     let mut lines = stdin.lines().cycle();
//     let mut numbers_seen: HashSet<i32> = HashSet::new();
//     numbers_seen.insert(0);
//     loop {
//         result += match lines.next().unwrap().parse::<i32>() {
//             Ok(x) => x,
//             Err(_) => 0,
//         };
//         if numbers_seen.contains(&result) {
//             break;
//         }
//         numbers_seen.insert(result);
//     }
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_sum_of_string() {
    //     assert_eq!(3, sum_of_string(&String::from("+1\n+1\n+1")));
    //     assert_eq!(0, sum_of_string(&String::from("+1\n+1\n-2")));
    //     assert_eq!(-6, sum_of_string(&String::from("-1\n-2\n-3")));
    // }

    // #[test]
    // fn test_first_reached_twice() {
    //     assert_eq!(0, first_reached_twice(&String::from("+1\n-1")));
    //     assert_eq!(10, first_reached_twice(&String::from("+3\n+3\n+4\n-2\n-4")));
    //     assert_eq!(5, first_reached_twice(&String::from("-6\n+3\n+8\n+5\n-6")));
    //     assert_eq!(14, first_reached_twice(&String::from("+7\n+7\n-2\n-7\n-4")));
    // }

}
