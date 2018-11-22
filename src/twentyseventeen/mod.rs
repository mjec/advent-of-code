mod day1;
mod day2;

make_dispatch!(
    "advent-of-code 2017",
    "1" => day1::dispatch => day1::dispatch_help => "The first day's challenges (captcha)",
    "2" => day2::dispatch => day2::dispatch_help => "The second day's challenges (spreadsheet checksum)"
);
