use return_codes;
use std::io::{self, Read};

make_dispatch_and_help!(
    "advent-of-code 2017 3",
    "1" => part1 => part1_help => "<num>" => "Part 1: number of taxicab steps to get from <num> back to 1 with a spiral structure",
    "2" => part2 => part2_help => "" => "Part 2: ???"
);

// 37  36  35  34  33  32  31
// 38  17  16  15  14  13  30
// 39  18   5   4   3  12  29
// 40  19   6   1   2  11  28
// 41  20   7   8   9  10  27
// 42  21  22  23  24  25  26
// 43  44  45  46  47  48  49

// 1   =>  0
// 3   =>  1
// 5   =>  2
// 7   =>  3
// ...

// "level_max_sqrt" = sqrt(n).ceil() + if that is even 1 else 0;
// "position" = n - (level_max_sqrt - 2)^2

// distance_inwards = (level_max_sqrt / 2).floor()
// distance_along_level =
// distance = distance_along_level + distance_inwards

fn part1(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 1, part1_help);

    Ok(())
}

fn part2(args: &mut Vec<String>) -> Result<(), i32> {
    // let argument_count = args.len();
    // if argument_count != 0 {
    //     return match part2_help(args) {
    //         Ok(()) => Err(return_codes::TOO_MANY_ARGUMENTS),
    //         e => e,
    //     };
    // }

    // let mut buffer = String::new();
    // if let Err(e) = io::stdin().read_to_string(&mut buffer) {
    //     println!("Unable to read from stdin: {:?}", e);
    //     return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
    // }

    // println!("{}", sum_of_possible_divisions_each_line(&buffer));
    Ok(())
}
