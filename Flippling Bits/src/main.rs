use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'flippingBits' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER n as parameter.
 */

fn flippingBits(n: i64) -> i64 {
   (i64::pow(2,32) - 1) - n
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();

        let result = flippingBits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
