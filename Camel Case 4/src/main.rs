mod lib;
use lib::*;

use std::io::{self, BufRead};

fn resolution(line: &String) -> String {
    // line == A;A;A
    let mut a: Vec<&str> = line.split(";").collect();
    a[2] = a[2].trim_end_matches("()");

    let ans = match a[0].to_uppercase().as_str() {
        "S" => separate_strings(a[2]),
        "C" => basic_combine(a[2], a[1]),
        _ => panic!("Error in options"),
    };


    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(input) = lines.next().map(|l| l.unwrap()) {
        println!("{}", resolution(&input));
    };
}
