mod day1;
mod day2;
mod day3;

make_dispatch!(
    "advent-of-code 2018",
    "1" => day1::dispatch => day1::dispatch_help => "The first day's challenges (sum of changes)",
    "2" => day2::dispatch => day2::dispatch_help => "The second day's challenges (box ID manipulation)",
    "3" => day3::dispatch => day3::dispatch_help => "The third day's challenges (claimed areas)"
);
