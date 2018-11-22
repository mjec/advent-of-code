#[macro_use]
mod make_dispatch;
mod return_codes;
mod twentyseventeen;

use std::env;

fn main() -> () {
    let mut args: Vec<String> = env::args().collect();
    ::std::process::exit(match dispatch(&mut args.split_off(1)) {
        Ok(_) => 0,
        Err(err) => err,
    })
}

make_dispatch!(
    "advent-of-code",
    "2017" => twentyseventeen::dispatch => twentyseventeen::dispatch_help => "The 2017 adevent of code challenges",
    "help" => dispatch_help => dispatch_help => "Information on each component"
);
