#[macro_use]
mod make_dispatch;
#[macro_use]
mod argument_count;
#[allow(dead_code)]
mod return_codes;
mod twentyeighteen;
mod twentyseventeen;

use std::env;

#[cfg(debug_assertions)]
use std::io;
#[cfg(debug_assertions)]
use std::io::prelude::*;
#[cfg(debug_assertions)]
extern crate termion;
#[cfg(debug_assertions)]
use termion::raw::IntoRawMode;

fn main() {
    if cfg!(debug_assertions) {
        println!("Press any key to continue...");
        wait_for_keypress();
    }
    let mut args: Vec<String> = env::args().collect();
    ::std::process::exit(match dispatch(&mut args.split_off(1)) {
        Ok(_) => 0,
        Err(err) => err,
    })
}

#[cfg(debug_assertions)]
fn wait_for_keypress() {
    let _stdout = io::stdout().into_raw_mode().unwrap(); // this has to stay in scope until end of function
    let _ = io::stdin().read(&mut [0u8]);
}
#[cfg(not(debug_assertions))]
#[inline]
fn wait_for_keypress() {}

make_dispatch!(
    "advent-of-code",
    "2017" => twentyseventeen::dispatch => twentyseventeen::dispatch_help => "The 2017 adevent of code challenges",
    "2018" => twentyeighteen::dispatch => twentyeighteen::dispatch_help => "The 2018 adevent of code challenges",
    "help" => dispatch_help => dispatch_help => "Information on each component"
);
