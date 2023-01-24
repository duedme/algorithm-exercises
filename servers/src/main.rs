use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    side: String,
    message: String,
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);
    println!("Side: {}, Message: {}", args.side, args.message);

    let mut file = File::open("/Users/Daniel/Aws/programa_que_escucha/server/messages.txt");
}