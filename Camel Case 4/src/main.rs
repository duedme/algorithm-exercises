use std::io::{self, BufRead};

fn resolucion(line: &String) {

}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(input) = lines.next().map(|l| l.unwrap()) {
        resolucion(&input);
    }
}
